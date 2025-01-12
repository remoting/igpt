use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub struct Error {
    pub code: i32,
    pub msg: String,
}

impl Error {
    // 新增构造函数
    pub fn new(code: i32, msg: &str) -> Self {
        Error {
            code,
            msg: msg.to_string(),
        }
    }
}

// 实现 fmt::Display trait 以便于打印错误信息
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error {}: {}", self.code, self.msg)
    }
}

// 实现 std::error::Error trait
impl std::error::Error for Error {}

impl From<Box<dyn std::error::Error>> for Error {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        Error {
            code: 500,
            msg: error.to_string(),
        }
    }
}

impl From<rusqlite::Error> for Error {
    fn from(error: rusqlite::Error) -> Self {
        Error {
            code: 500,
            msg: error.to_string(),
        }
    }
}
impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error {
            code: 500,
            msg: error.to_string(),
        }
    }
}
