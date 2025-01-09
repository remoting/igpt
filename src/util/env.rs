use lazy_static::lazy_static;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::Manager;
lazy_static! {
    static ref APP_INSTANCE: Mutex<Option<tauri::AppHandle>> = Mutex::new(None);
    static ref DOCUMENT_DIR: Mutex<Option<String>> = Mutex::new(None);
}

pub fn get_app_handle() -> Option<tauri::AppHandle> {
    let instance = APP_INSTANCE.lock().unwrap();
    instance.clone()
}
pub fn set_app_handle(app_handle: tauri::AppHandle) {
    let mut instance = APP_INSTANCE.lock().unwrap();
    *instance = Some(app_handle.clone());
}

pub fn document_dir() -> PathBuf {
    #[cfg(test)]
    {
        PathBuf::from("/Users/lanren/Documents/igpt")
    }
    #[cfg(not(test))]
    {
        if let Some(app) = get_app_handle() {
            match app.path().document_dir() {
                Ok(path) => {
                    let mut doc_dir = path;
                    // 检查是否是 mobile，如果不是，则在 doc_dir 后面添加 "igpt"
                    #[cfg(not(mobile))]
                    {
                        doc_dir = doc_dir.join("igpt");
                    }
                    if !doc_dir.exists() {
                        if let Err(e) = fs::create_dir_all(&doc_dir) {
                            eprintln!("无法创建 logs 目录: {}", e);
                        }
                    }
                    doc_dir
                }
                Err(_e) => PathBuf::new(),
            }
        } else {
            PathBuf::new()
        }
    }
}
pub fn get_logs_dir() -> String {
    let doc_dir = document_dir();

    let logs_dir = doc_dir.join("logs");
    // 检查 logs 目录是否存在，如果不存在则创建
    if !logs_dir.exists() {
        if let Err(e) = fs::create_dir_all(&logs_dir) {
            eprintln!("无法创建 logs 目录: {}", e);
        }
    }
    return logs_dir.to_string_lossy().to_string();
}

pub fn get_plaform() -> String {
    let platform = match std::env::consts::OS {
        "windows" => "Windows",
        "macos" => "macOS",
        "linux" => "Linux",
        other => other, // 处理其他平台
    };
    return platform.to_string();
}
pub fn get_arch() -> String {
    let arch = std::env::consts::ARCH;
    return arch.to_string();
} 