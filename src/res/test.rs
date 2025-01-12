#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{res::app_version_upgrade, util::env};

    // 导入父模块的所有项，使得测试模块可以访问它们
    use super::*;
    // 一个简单的测试函数
    #[test]
    fn test_add() {
        let mut path = PathBuf::from("/Users/lanren/codeup/keeyuu/igpt/tauri");
        println!("{}", path.to_string_lossy().to_string());
        path.push("ui");
        if path.is_dir() {
            path.push("index.html");
        }
        println!("x{}x", path.to_str().unwrap_or(""));
    }
    #[test]
    fn test_upgrade() -> Result<(), crate::util::error::Error> {
        app_version_upgrade("v1.0.0", "https://www.keeyuu.com/version/v1.0.0.zip")
    }
    #[test]
    fn test() -> Result<(), crate::util::error::Error> {
        env::document_dir();
        Ok(())
    }
}
