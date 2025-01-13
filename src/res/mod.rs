use std::fs::File;
use std::io::Read;
use tauri::http::status::StatusCode;
use tauri::http::Request;
use tauri::http::Response;
use reqwest::blocking::get;
use crate::db;
use crate::db::sqlite;
use crate::util;
use crate::util::env::document_dir;
use crate::util::env::get_temp_dir;
use crate::util::error::Error;
use crate::util::json::Json;
use chrono::prelude::*;
use mime_guess::from_path;
use std::path::PathBuf;

pub fn get_http_response(request: Request<Vec<u8>>) -> Response<Vec<u8>> {
    let mut _path = request.uri().path();
    if _path.starts_with("/v") {
        let mut path = document_dir();
        _path = _path.strip_prefix("/").unwrap_or(_path);
        path.push(_path);
        if path.is_dir() {
            path.push("index.html");
        }
        match File::open(&path) {
            Ok(mut file) => {
                let mut contents: Vec<u8> = Vec::new();
                let mime_type = from_path(&path).first_or_octet_stream();
                match file.read_to_end(&mut contents) {
                    Ok(_e) => {
                        Response::builder()
                        .status(StatusCode::OK)
                        .header("Content-Type", mime_type.as_ref())
                        .header("Content-Length", _e.to_string())
                        .body(contents)
                        .unwrap_or_default()
                    }
                    Err(_e) => {
                        Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(format!("Failed to read file: {}", _e).into())
                        .unwrap_or_default()
                    }
                }
            }
            Err(err) => {
                Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(format!("Failed to open file: {}", err).into())
                .unwrap_or_default()
            }
        }
    } else {
        Response::builder()
            .status(404)
            .body("Not Found".into())
            .unwrap_or_default()
    }
}

#[tauri::command(name = "app_version")]
pub fn app_version() -> Result<String, Error> {
    let mut version = super::db::config("ui_version");
    if version == "" {
        version = app_version_init()?
    }
    Ok(version)
}

#[tauri::command(name = "app_upgrade")]
pub fn app_upgrade(version: &str, url: &str) -> Result<String, Error> {
    app_version_upgrade(version, url)?;
    app_version()
}
fn app_version_init() -> Result<String,Error>{
    let latest = "https://www.keeyuu.com/app/igpt/latest.json";
    let response = get(latest)?;
    let info: Json = response.json()?;
    if info.contains("version") && info.contains("url") {
        let version = info.get_str("version");
        let url = info.get_str("url");
        app_version_upgrade(&version, &url)?;
        Ok(info.get_str("version"))
    }else{
        Err(Error::new(1, "msg"))
    } 
}
fn app_version_upgrade(version: &str, url: &str) -> Result<(), Error> {
    // download
    let now: DateTime<Utc> = Utc::now();
    // 将时间格式化为字符串
    let formatted_time = now.format("%Y%m%d%H%M%S").to_string();
    let mut path = PathBuf::from(get_temp_dir());
    path.push(format!("{}.zip", formatted_time));
    crate::util::file::download_file(url, &path)?;
    // 版本目录
    let mut vpath = document_dir();
    vpath.push(format!("{}", version));
    if vpath.exists() {
        std::fs::remove_dir_all(&vpath)?;
    }
    // 解压
    crate::util::file::unzip_file(&path, &vpath)?;
    // 删除临时文件
    std::fs::remove_file(&path)?;
    vpath.push("schema.sql");
    // 数据库结构更新
    let sql = util::file::read_file(&vpath)?;
    db::sqlite_migrate(&sql, None)?;
    // 更新数据库
    let params = vec![
        serde_json::Value::String("ui_version".to_string()),
        serde_json::Value::String(version.to_string()),
    ];
    sqlite::exec("INSERT INTO config (conf_key, conf_val) VALUES (?, ?) ON CONFLICT(conf_key) DO UPDATE SET conf_val = excluded.conf_val",  Some(params))?;
    let params = vec![
        serde_json::Value::String("ui_url".to_string()),
        serde_json::Value::String(url.to_string()),
    ];
    sqlite::exec("INSERT INTO config (conf_key, conf_val) VALUES (?, ?) ON CONFLICT(conf_key) DO UPDATE SET conf_val = excluded.conf_val",  Some(params))?;
    // 更新缓存数据
    db::load_config();
    Ok(())
}
#[cfg(test)]
mod test;
