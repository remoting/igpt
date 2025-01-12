use super::super::util::error::Error;
use lazy_static::lazy_static;
use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::ToSql;
use serde_json::{json, Value};
use std::sync::Arc;

lazy_static! {
    static ref DB_POOL: Arc<Pool<SqliteConnectionManager>> = {
        let doc_dir = crate::util::env::document_dir();
        let manager = SqliteConnectionManager::file(
            doc_dir.join("database.db").to_string_lossy().to_string(),
        );
        Arc::new(Pool::new(manager).expect("Failed to create pool"))
    };
}
pub fn get_conn() -> Result<PooledConnection<SqliteConnectionManager>, Error> {
    let pool = DB_POOL.clone();
    pool.get().map_err(|e| Error {
        code: 500,
        msg: e.to_string(),
    })
}
fn convert_params(params: Option<Vec<Value>>) -> Result<Vec<Box<dyn ToSql>>, Error> {
    params.map_or(Ok(Vec::new()), |params| {
        params
            .into_iter()
            .map(|p| {
                Ok(match p {
                    Value::String(s) => Box::new(s) as Box<dyn ToSql>,
                    Value::Number(n) => {
                        if let Some(i) = n.as_i64() {
                            Box::new(i) as Box<dyn ToSql>
                        } else if let Some(f) = n.as_f64() {
                            Box::new(f) as Box<dyn ToSql>
                        } else {
                            return Err(Error {
                                code: 500,
                                msg: "Unsupported number type".to_string(),
                            });
                        }
                    }
                    Value::Bool(b) => Box::new(b) as Box<dyn ToSql>,
                    Value::Null => Box::new(rusqlite::types::Null) as Box<dyn ToSql>,
                    _ => {
                        return Err(Error {
                            code: 500,
                            msg: "Unsupported type".to_string(),
                        })
                    }
                })
            })
            .collect()
    })
}

pub fn exec(sql: &str, params: Option<Vec<Value>>) -> Result<Value, Error> {
    let conn = get_conn()?;
    let params = convert_params(params)?;
    let param_refs: Vec<&dyn ToSql> = params.iter().map(|p| p.as_ref()).collect();

    let affected_rows = conn.execute(sql, &param_refs[..]).map_err(|e| Error {
        code: 500,
        msg: e.to_string(),
    })?;
    let last_inserted_id: i64 = conn.last_insert_rowid();
    Ok(json!({ "rows": affected_rows, "rowid": last_inserted_id }))
}

pub fn query(sql: &str, params: Option<Vec<Value>>) -> Result<Value, Error> {
    let conn = get_conn()?;
    let mut stmt = conn.prepare(sql)?;
    // Extract column names and drop the immutable reference
    let column_names: Vec<String> = stmt.column_names().iter().map(|s| s.to_string()).collect();
    // 执行并返回
    let params1 = convert_params(params)?;
    let param_refs: Vec<&dyn ToSql> = params1.iter().map(|p| p.as_ref()).collect();
    let mut rows = stmt.query(&param_refs[..])?;

    let mut result = Vec::new();
    while let Some(row) = rows.next()? {
        let mut row_map = serde_json::Map::new();
        for (i, column) in column_names.iter().enumerate() {
            //let value: rusqlite::Result<rusqlite::types::Value> = row.get(i);
            match row.get::<_, rusqlite::types::Value>(i) {
                Ok(value) => {
                    // 将 rusqlite::types::Value 转换为 serde_json::Value
                    let json_value = match value {
                        rusqlite::types::Value::Null => json!(null),
                        rusqlite::types::Value::Integer(i) => json!(i),
                        rusqlite::types::Value::Real(f) => json!(f),
                        rusqlite::types::Value::Text(s) => json!(s),
                        rusqlite::types::Value::Blob(b) => json!(b),
                    };
                    row_map.insert(column.to_string(), json_value);
                }
                Err(e) => {
                    return Err(Error {
                        code: 500,
                        msg: e.to_string(),
                    });
                }
            }
        }
        result.push(Value::Object(row_map));
    }
    if result.is_empty() {
        Ok(Value::Array(vec![])) // 返回空数组
    } else {
        Ok(Value::Array(result)) // 返回非空数组
    }
}
