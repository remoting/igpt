use super::super::util::error::Error;
use super::super::util::json::Json;
use lazy_static::lazy_static;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Mutex;
pub type CommandHandler = Box<dyn Fn(Json) -> Result<Json,Error> + Send + 'static>;

#[derive(Serialize)] 
pub struct Response {
    code: i32,
    msg: String,
    data: Option<Json>, // data 是可选的
}
// 定义一个全局的命令注册表
lazy_static! {
    pub static ref COMMAND_REGISTRY: Mutex<HashMap<String, CommandHandler>> = Mutex::new(HashMap::new()); 
    pub(crate) static ref INIT: () = {
         lazy_static::initialize(&crate::controller::home::HANDLER_INIT);
         lazy_static::initialize(&crate::controller::user::HANDLER_INIT);
    };
}

// 自定义一个宏，在某个函数上使用这个宏，宏的功能是调用registry_command函数注册这个函数
#[macro_export]
macro_rules! handler_registry {
    ($handler:expr) => {
        lazy_static::lazy_static! {
            pub(crate) static ref HANDLER_INIT: () = {
                let module_path = module_path!().replace("::", "/");
                let function_name = stringify!($handler);
                let command_name = format!("{}/{}", module_path, function_name);
                $crate::controller::base::register_command(
                    command_name,
                    Box::new($handler)
                ).expect(&format!("Failed to register command: {}", function_name));
            };
        }
    };
}

// 注册命令的函数
pub fn register_command(name: String, callback_fn: CommandHandler) -> Result<(), String> {
    COMMAND_REGISTRY
        .lock()
        .map_err(|e| format!("Failed to lock registry: {}", e))?
        .insert(name, callback_fn);
    Ok(())
}