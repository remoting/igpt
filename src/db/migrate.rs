use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rand::Rng;
use regex::Regex;
use rusqlite::params;
use std::collections::HashMap;
#[derive(Debug)]
struct Column {
    name: String,
    column_type: String,
    is_primary_key: bool,
    is_auto_increment: bool,
    is_not_null: bool,
    default_value: Option<String>,
}

#[derive(Debug)]
struct Table {
    name: String,
    columns: Vec<Column>,
    statement: String,
}

#[derive(Debug)]
struct Index {
    name: String,
    table_name: String,
    columns: Vec<String>,
    statement: String,
}
fn generate_random_number(min: u32, max: u32) -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..=max) // 使用包含上限的范围
}
fn sanitize_identifier(identifier: &str) -> String {
    identifier
        .trim_matches(|c| c == '"' || c == '\'')
        .to_string()
}
fn parse_create_table_statement(statement: &str) -> Table {
    let table_name_regex =
        Regex::new(r#"(?i)CREATE TABLE\s+"?([^".\s]+)"?(?:\.\s*"([^"\s]+)")?"#).unwrap();
    //let table_name_regex = Regex::new(r#"CREATE TABLE\s+"?([^".\s]+)"?(?:\.\s*"([^"\s]+)")?"#).unwrap();
    let column_definition_regex = Regex::new(r"\(([^)]+)\)").unwrap();
    //let column_definition_regex = Regex::new(r"\(([^)]+)\)").unwrap();
    let table_name = if let Some(caps) = table_name_regex.captures(statement) {
        if let Some(matched_name) = caps.get(2) {
            sanitize_identifier(matched_name.as_str())
        } else {
            sanitize_identifier(caps.get(1).unwrap().as_str())
        }
    } else {
        panic!("Failed to capture table name");
    };

    let column_definitions = column_definition_regex
        .captures(statement)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str();

    let columns = column_definitions
        .split(',')
        .filter(|def| {
            let name = def.trim();
            if name.is_empty() {
                return false;
            }
            if name.to_uppercase().starts_with("PRIMARY KEY")
                || name.to_uppercase().starts_with("UNIQUE")
            {
                return false;
            }
            true
        })
        .map(|def| {
            let parts: Vec<&str> = def.trim().split_whitespace().collect();
            let name = sanitize_identifier(parts[0]);
            let column_type = parts[1].to_string();
            let constraints = parts[2..].join(" ");
            let is_primary_key = constraints.contains("PRIMARY KEY");
            let is_auto_increment = constraints.contains("AUTOINCREMENT");
            let is_not_null = constraints.contains("NOT NULL");
            let default_value = Regex::new(r"DEFAULT\s+(\S+)")
                .unwrap()
                .captures(&constraints)
                .map(|caps| caps.get(1).unwrap().as_str().to_string());

            Column {
                name,
                column_type,
                is_primary_key,
                is_auto_increment,
                is_not_null,
                default_value,
            }
        })
        .collect();

    Table {
        name: table_name.to_string(),
        columns,
        statement: statement.to_string(),
    }
}

fn parse_create_index_statement(statement: &str) -> Index {
    let index_regex = Regex::new(r#"(?i)CREATE INDEX\s+("([^"]+)"\."([^"]+)"|"([^"]+)")\s+ON\s+("([^"]+)"\."([^"]+)"|"([^"]+)")\s*\(([^)]+)\)"#).unwrap();
    //let index_regex = Regex::new(r#"CREATE INDEX\s+(?:(['"])([^'"]+)\1\.)?(['"])([^'"]+)\3\s+ON\s+(?:(['"])([^'"]+)\5\.)?(['"])([^'"]+)\7\s*\(([^)]+)\)"#).unwrap();
    //let index_regex = Regex::new(r#"CREATE INDEX\s+("([^"]+)"|'([^']+)')\s+ON\s+("([^"]+)"|'([^']+)')\s*\(([^)]+)\)"#).unwrap();

    let captures = index_regex
        .captures(statement)
        .expect("Failed to parse CREATE INDEX statement");

    let index_name = if let Some(_schema_name) = captures.get(2) {
        sanitize_identifier(captures.get(3).unwrap().as_str())
    } else {
        sanitize_identifier(captures.get(4).unwrap().as_str())
    };

    let table_name = if let Some(_schema_name) = captures.get(6) {
        sanitize_identifier(captures.get(7).unwrap().as_str())
    } else {
        sanitize_identifier(captures.get(8).unwrap().as_str())
    };

    let columns_str = captures.get(9).unwrap().as_str();

    let columns: Vec<String> = columns_str
        .split(',')
        .map(|col| sanitize_identifier(col.trim()))
        .collect();

    Index {
        name: index_name,
        table_name,
        columns,
        statement: statement.to_string(),
    }
}

fn parse_sql_file(file_content: &str) -> (Vec<Table>, Vec<Index>) {
    let sql_content = file_content.to_string();
    let statements: Vec<&str> = sql_content
        .split(';')
        .map(|stmt| stmt.trim())
        .filter(|stmt| !stmt.is_empty())
        .collect();

    let create_table_statements: Vec<&str> = statements
        .iter()
        .filter(|stmt| stmt.to_uppercase().starts_with("CREATE TABLE"))
        .map(|stmt| *stmt)
        .collect();

    let create_index_statements: Vec<&str> = statements
        .iter()
        .filter(|stmt| stmt.to_uppercase().starts_with("CREATE INDEX"))
        .map(|stmt| *stmt)
        .collect();

    let tables = create_table_statements
        .iter()
        .map(|stmt| parse_create_table_statement(stmt))
        .collect();

    let indices = create_index_statements
        .iter()
        .map(|stmt| parse_create_index_statement(stmt))
        .collect();

    (tables, indices)
}

fn get_database_tables(
    conn: &PooledConnection<SqliteConnectionManager>,
) -> Result<Vec<Table>, Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table'")?;
    let table_names: Vec<String> = stmt
        .query_map(params![], |row| Ok(row.get(0)?))?
        .collect::<Result<Vec<String>, _>>()?;

    let mut tables = Vec::new();

    for table_name in table_names {
        let mut stmt = conn.prepare(&format!("PRAGMA table_info({})", table_name))?;
        let columns = stmt
            .query_map(params![], |row| {
                Ok(Column {
                    name: row.get(1)?,
                    column_type: row.get(2)?,
                    is_primary_key: row.get(5)?,
                    is_auto_increment: false, // SQLite doesn't provide this info directly
                    is_not_null: row.get(3)?,
                    default_value: row.get(4)?,
                })
            })?
            .collect::<Result<Vec<Column>, _>>()?;

        tables.push(Table {
            name: table_name,
            columns,
            statement: "".to_string(),
        });
    }

    Ok(tables)
}

fn get_database_indices(
    conn: &PooledConnection<SqliteConnectionManager>,
) -> Result<Vec<Index>, Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare("SELECT name,tbl_name FROM sqlite_master WHERE type='index'")?;

    let index_map_iter = stmt.query_map(params![], |row| {
        let mut map = HashMap::new();
        map.insert("name".to_string(), row.get::<_, String>(0)?);
        map.insert("tbl_name".to_string(), row.get::<_, String>(1)?);
        Ok(map)
    })?;

    let mut indices = Vec::new();

    for index_map in index_map_iter {
        let index = index_map?;
        let index_name = index.get("name").ok_or("name not found")?;
        let table_name = index.get("tbl_name").ok_or("tbl_name not found")?;

        let mut stmt = conn.prepare(&format!("PRAGMA index_info({})", index_name))?;
        let columns = stmt
            .query_map(params![], |row| Ok(row.get(2)?))?
            .collect::<Result<Vec<String>, _>>()?;

        // let mut stmt = conn.prepare(&format!("PRAGMA index_list({})", index_name))?;
        // let table_name: String = stmt.query_map(params![], |row| Ok(row.get(1)?))?.next().unwrap()?;

        indices.push(Index {
            name: index_name.clone(),
            table_name: table_name.clone(),
            columns,
            statement: "".to_string(),
        });
    }

    Ok(indices)
}

pub fn update_database_structure(
    conn: &PooledConnection<SqliteConnectionManager>,
    sql_content: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let (sql_tables, sql_indices) = parse_sql_file(sql_content);
    let db_tables = get_database_tables(conn)?;

    // 处理表
    for sql_table in sql_tables {
        let db_table = db_tables.iter().find(|t| t.name == sql_table.name);

        if db_table.is_none() {
            // 表不存在，创建表
            conn.execute(&sql_table.statement, params![])?;
            println!("Table '{}' created.", sql_table.name);
        } else {
            // 表存在，检查字段
            let db_table_columns: &Vec<Column> = &db_table.unwrap().columns;

            let mut requires_migration = false;

            for sql_column in &sql_table.columns {
                let db_column = db_table_columns
                    .iter()
                    .find(|col| col.name == sql_column.name);

                if db_column.is_none() {
                    // 字段不存在，添加字段
                    let stmt = format!(
                        "ALTER TABLE {} ADD COLUMN {} {} {} {}",
                        sql_table.name,
                        sql_column.name,
                        sql_column.column_type,
                        if sql_column.is_not_null {
                            "NOT NULL"
                        } else {
                            ""
                        },
                        sql_column
                            .default_value
                            .as_ref()
                            .map_or("".to_string(), |v| format!("DEFAULT {}", v))
                    );
                    conn.execute(&stmt, params![])?;
                    println!(
                        "Column '{}' added to table '{}'.",
                        sql_column.name, sql_table.name
                    );
                } else {
                    let db_column = db_column.unwrap();
                    if db_column.column_type.to_uppercase() != sql_column.column_type.to_uppercase()
                        || db_column.is_primary_key != sql_column.is_primary_key
                       // || db_column.is_auto_increment != sql_column.is_auto_increment
                        || db_column.is_not_null != sql_column.is_not_null
                        || db_column.default_value != sql_column.default_value
                    {
                        // 字段类型、主键、AUTOINCREMENT、NOT NULL或默认值不一致，需要迁移
                        requires_migration = true;
                    }
                }
            }

            if requires_migration {
                // 创建新表
                let new_table_name = format!(
                    "{}_new_{}",
                    sql_table.name,
                    generate_random_number(10000, 99999)
                );
                let create_table_statement = sql_table
                    .statement
                    .replace(&sql_table.name, &new_table_name);
                conn.execute(&create_table_statement, params![])?;
                println!("New table '{}' created.", new_table_name);

                // 迁移数据
                let common_columns: Vec<&String> = db_table_columns
                    .iter()
                    .filter(|col| {
                        sql_table
                            .columns
                            .iter()
                            .any(|sql_col| sql_col.name == col.name)
                    })
                    .map(|col| &col.name)
                    .collect();

                let column_list = common_columns
                    .iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<&str>>()
                    .join(", ");
                let stmt = format!(
                    "INSERT INTO {} ({}) SELECT {} FROM {}",
                    new_table_name, column_list, column_list, sql_table.name
                );
                conn.execute(&stmt, params![])?;
                println!(
                    "Data migrated from '{}' to '{}'.",
                    sql_table.name, new_table_name
                );

                // 删除旧表
                let stmt = format!("DROP TABLE {}", sql_table.name);
                conn.execute(&stmt, params![])?;
                println!("Old table '{}' dropped.", sql_table.name);

                // 重命名新表
                let stmt = format!(
                    "ALTER TABLE {} RENAME TO {}",
                    new_table_name, sql_table.name
                );
                conn.execute(&stmt, params![])?;
                println!(
                    "New table '{}' renamed to '{}'.",
                    new_table_name, sql_table.name
                );
            }
        }
    }

    let db_indices = get_database_indices(conn)?;
    // 处理索引
    for sql_index in sql_indices {
        let db_table = db_tables.iter().find(|t| t.name == sql_index.table_name);
        // 如果找到索引需要的表
        if let Some(_db_table) = db_table {
            let db_index = db_indices.iter().find(|idx| idx.name == sql_index.name);

            // 如果没有在数据库中找到索引就创建
            if db_index.is_none() {
                // 索引不存在，创建索引
                conn.execute(&sql_index.statement, params![])?;
                println!(
                    "Index '{}' created on table '{}'.",
                    sql_index.name, sql_index.table_name
                );
            } else {
                let db_index = db_index.unwrap();
                if db_index.columns != sql_index.columns {
                    // 索引列不一致，重新创建索引
                    let stmt = format!("DROP INDEX {}", sql_index.name);
                    conn.execute(&stmt, params![])?;
                    println!("Index '{}' dropped.", sql_index.name);
                    conn.execute(&sql_index.statement, params![])?;
                    println!(
                        "Index '{}' created on table '{}'.",
                        sql_index.name, sql_index.table_name
                    );
                }
            }
        }
    }

    conn.execute("VACUUM", params![])?;
    conn.execute("ANALYZE", params![])?;
    conn.execute("REINDEX", params![])?;
    conn.execute("PRAGMA optimize", params![])?;
    Ok(())
}
