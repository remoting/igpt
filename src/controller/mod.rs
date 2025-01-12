use crate::util::error::Error;
use crate::util::json::Json;
use log::info;
pub mod base;
mod home;
mod user;

// 获取命令注册表的命令名
#[tauri::command]
pub fn get_command_registry() -> Vec<String> {
    let registry = base::COMMAND_REGISTRY.lock().unwrap();
    registry.keys().cloned().collect()
}

// 样例命令
#[tauri::command]
pub async fn example_command(args: Json) -> Result<Json, Error> {
    info!("example_command: {:?}", args);
    Ok(args)
}
//动态调用命令的函数
#[tauri::command]
pub async fn invoke_dynamic_command(name: String, args: Json) -> Result<Json, Error> {
    let registry = base::COMMAND_REGISTRY.lock().unwrap();
    if let Some(callback) = registry.get(&name) {
        match callback(args) {
            Ok(data) => {
                let mut resp = Json::empty();
                resp.set_int("code", 0);
                resp.set_str("msg", "");
                resp.set_j("data", data);
                Ok(resp)
            }
            Err(e) => {
                let mut resp = Json::empty();
                resp.set_int("code", e.code.into());
                resp.set_str("msg", &e.msg);
                Ok(resp)
            }
        }
    } else {
        Err(Error::new(404, "Command not found"))
    }
}
