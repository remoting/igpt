
#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    // 导入父模块的所有项，使得测试模块可以访问它们
    use super::*;
    // 一个简单的测试函数
    #[test]
    fn test_add() {
        let mut path = PathBuf::from("/Users/lanren/codeup/keeyuu/igpt/tauri");
        println!("{}",path.to_string_lossy().to_string());
        path.push("ui");
        if path.is_dir() { 
            path.push("index.html");
        }
        println!("x{}x",path.to_str().unwrap_or("")); 
    }
 
}