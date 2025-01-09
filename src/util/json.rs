use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::ops::Deref;
#[derive(Serialize, Deserialize, Debug)]
pub struct Json(Value); // 封装 serde_json::Value

impl Json {
    pub fn new(data: Value) -> Self {
        Json(data)
    }

    pub fn empty() -> Self {
        Json(Value::Object(serde_json::Map::new()))
    }

    pub fn get_str(&self, key: &str) -> String {
        match self.0.get(key) {
            Some(value) => value.as_str().unwrap_or("").to_string(),
            None => "".to_string(),
        }
    }

    pub fn get_int(&self, key: &str) -> i64 {
        match self.0.get(key) {
            Some(value) => value.as_i64().unwrap_or(0),
            None => 0,
        }
    }

    pub fn set_str(&mut self, key: &str, value: &str) {
        self.0[key] = Value::String(value.to_string());
    }

    pub fn set_int(&mut self, key: &str, value: i64) {
        self.0[key] = Value::Number(value.into());
    }
    pub fn set_v(&mut self, key: &str, data: Value) {
        self.0[key] = data;
    }
    pub fn set_j(&mut self, key: &str, data: Json) {
        self.0[key] = data.0;
    }

    pub fn contains(&self, key: &str) -> bool {
        self.0.get(key).is_some()
    }
}

impl Deref for Json {
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
// 在这里添加一个函数，输入一个字符串，返回一个 Json 结构体
pub fn from_str(json_str: &str) -> Result<Json, serde_json::Error> {
    let data: Value = serde_json::from_str(json_str)?;
    Ok(Json::new(data))
}

// 在这里添加一个函数，输入一个 Json 结构体，返回一个字符串
pub fn to_string(json: &Json) -> Result<String, serde_json::Error> {
    serde_json::to_string(&json.0)
}
