use std::fs::File;
use std::io::Read;
use tauri::http::Request;
use tauri::http::Response;
use tauri::http::{header::*, response::Builder as ResponseBuilder, status::StatusCode};

use crate::util::env::document_dir;
use crate::util::error::Error;
use mime_guess::from_path;
use std::path::{Path, PathBuf};
//let mut resp = ResponseBuilder::new();
//let mut resp = ResponseBuilder::new();
// let mut resp = ResponseBuilder::new()
//                 .status(StatusCode::INTERNAL_SERVER_ERROR)
//                 .header(CONTENT_TYPE, "text/plain")
//                 .body("e.to_string()".as_bytes().to_vec()).unwrap();

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

#[tauri::command(name = "ui_version")]
pub fn ui_version() -> Result<String, Error> {
    let mut version = super::db::config("ui_version");
    if version == "" {
        version = "v1.0.0".to_string()
    }
    Ok(version)
}

#[cfg(test)]
mod test;
