use std::fs::File;
use std::io::Read;
use tauri::http::Request;
use tauri::http::Response;
use tauri::http::{header::*, response::Builder as ResponseBuilder, status::StatusCode};
 
use chrono::prelude::*;
use crate::db;
use crate::util;
use crate::util::env::document_dir;
use crate::util::env::get_temp_dir;
use crate::util::error::Error;
use mime_guess::from_path;
use std::path::PathBuf;
use crate::db::sqlite;

pub fn get_http_response(request: Request<Vec<u8>>) -> Response<Vec<u8>> {
    let mut _path = request.uri().path();
    if _path.starts_with("/v") {
        let mut path = document_dir();
        _path = _path.strip_prefix("/").unwrap_or(_path);
        path.push(_path);
        if path.is_dir() {
            path.push("index.html");
        }
        if let Ok(mut file) = File::open(&path) {
            let mut contents: Vec<u8> = Vec::new();
            let mime_type = from_path(&path).first_or_octet_stream();
            if let Ok(_e) = file.read_to_end(&mut contents) {
                return Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-Type", mime_type.as_ref())
                    .header("Content-Length", _e.to_string())
                    .body(contents)
                    .unwrap_or_default();
            }
        }
        Response::builder()
            .status(404)
            .body("Not Found".into())
            .unwrap_or_default()
    } else {
        Response::builder()
            .status(404)
            .body("Not Found".into())
            .unwrap_or_default()
    }
}

#[tauri::command(name = "app_version")]
pub fn app_version() -> Result<String, Error> {
    let version = super::db::config("ui_version");
    if version == "" {
        return Err(Error::new(1, "未初始化"));
    }
    Ok(version)
}

#[tauri::command(name = "app_upgrade")]
pub fn app_upgrade(version:&str,url:&str) -> Result<String, Error> {
    app_version_upgrade(version, url)?;
    app_version()
}

fn app_version_upgrade(version:&str,url:&str) -> Result<(), Error> {
    // download
    let now: DateTime<Utc> = Utc::now();
    // 将时间格式化为字符串
    let formatted_time = now.format("%Y%m%d%H%M%S").to_string();
    let mut path = PathBuf::from(get_temp_dir()); 
    path.push(format!("{}.zip",formatted_time));
    crate::util::file::download_file(url, &path)?;
    // 版本目录
    let mut vpath = document_dir();
    vpath.push(format!("{}",version));
    if (vpath.exists()) {
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
        serde_json::Value::String(version.to_string()),
        serde_json::Value::String("ui_version".to_string())
    ];
    sqlite::exec("INSERT INTO config (conf_key, conf_val) VALUES (?, ?) ON CONFLICT(conf_key) DO UPDATE SET conf_val = excluded.conf_val",  Some(params))?;
    // 更新缓存数据
    db::load_config();
    Ok(())
}
#[cfg(test)]
mod test;
