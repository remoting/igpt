use super::util::error::Error;
use crate::util;
use log::info;
use log::warn;
use log::LevelFilter;
use log4rs::{
    append::console::ConsoleAppender,
    append::file::FileAppender,
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};
use serde_json::Value;
use serde_json::to_string;

#[tauri::command(name = "log4rs_info")]
pub fn log4rs_info(params: Option<Vec<Value>>) -> Result<Value, Error> {
    let log_msg = log_message(params);
    // 直接使用 log_msg，确保没有转义字符
    info!("javascript: {}", log_msg);
    Ok("ok".into())
}

#[tauri::command(name = "log4rs_warn")]
pub fn log4rs_warn(params: Option<Vec<Value>>) -> Result<Value, Error> {
    let log_msg = log_message(params);
    // 直接使用 log_msg，确保没有转义字符
    warn!("javascript: {}", log_msg);
    Ok("ok".into())
}

pub fn log_message(params: Option<Vec<Value>>) -> String {
    if let Some(values) = params {
        // 序列化每个值并用逗号拼接
        let serialized_values: Vec<String> = values.iter()
            .filter_map(|value| to_string(value).ok()) // 序列化每个值，过滤掉失败的情况
            .collect();

        // 用逗号拼接序列化后的字符串
        return serialized_values.join(", ");
    } else {
        String::new() // 如果没有参数，返回空字符串
    }
}
pub fn init_log4rs() {
    let path = std::path::PathBuf::from(util::env::get_logs_dir());
    // 创建文件 appender
    let file_appender = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)} [{l}] {f}:{L} - {m}{n}",
        )))
        .build(path.join("output.log"))
        .unwrap();

    // 创建控制台 appender
    let console_appender = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)} [{l}] {f}:{L} - {m}{n}",
        )))
        .build();

    // 创建配置
    let config = Config::builder()
        .appender(Appender::builder().build("file", Box::new(file_appender)))
        .appender(Appender::builder().build("console", Box::new(console_appender)))
        .build(
            Root::builder()
                .appender("file")
                .appender("console")
                .build(LevelFilter::Info),
        )
        .unwrap();

    // 初始化 log4rs
    log4rs::init_config(config).unwrap();
    log::info!("系统启动")
}
