pub mod sqlite;
mod migrate;
use crate::util::json::Json;
use serde_json::{Value,json};

use super::util::error::Error;
use lazy_static::lazy_static;
use std::sync::Mutex;
lazy_static! {
    static ref APP_CONFIG: Mutex<Option<Json>> = Mutex::new(None);
}
pub fn load_config() {
    let rows = sqlite::query("select * from config", None);
    match rows {
        Ok(_value) => {
            let mut config = Json::empty();
            for row in _value.as_array().unwrap() {
                let conf_key = row.get("conf_key").unwrap().as_str().unwrap();
                let conf_val = row.get("conf_val").unwrap().as_str().unwrap();

                config.set_str(conf_key, conf_val);
            }

            *APP_CONFIG.lock().unwrap() = Some(config);
        }
        Err(error) => {
            log::info!("加载配置失败:{}", error)
        }
    }
}
#[tauri::command(name = "sqlite_exec")]
pub fn sqlite_exec(sql: &str, params: Option<Vec<Value>>) -> Result<Value, Error> {
    sqlite::exec(sql, params)
}

#[tauri::command(name = "sqlite_query")]
pub fn sqlite_query(sql: &str, params: Option<Vec<Value>>) -> Result<Value, Error> {
    sqlite::query(sql, params)
}

#[tauri::command(name = "sqlite_migrate")]
pub fn sqlite_migrate(sql: &str, _params: Option<Vec<Value>>) -> Result<Value, Error> {
    let conn = sqlite::get_conn()?;
    migrate::update_database_structure(&conn, sql)?;
    Ok(json!("ok")) 
}

pub fn config(key: &str) -> String {
    let instance = APP_CONFIG.lock().unwrap();
    if let Some(config) = instance.as_ref() {
        config.get_str(key)
    } else {
        "".to_string()
    }
}
#[cfg(test)]
mod test;
