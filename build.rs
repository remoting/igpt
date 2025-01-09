use std::process::Command;
fn main() {
    // 检查构建配置，如果是 release 模式，则传递 -s 参数
    if std::env::var("PROFILE").unwrap() == "release" {
        println!("cargo:rustc-link-arg=-s");
        // 执行 Tauri 的构建
        tauri_build::build();

        // 调用 strip 命令进一步减小二进制文件大小
        #[cfg(target_os = "linux")]
        {
            strip_binary("target/release/igpt");
        }

        #[cfg(target_os = "macos")]
        {
            strip_binary("target/release/igpt");
        }

        #[cfg(target_os = "windows")]
        {
            strip_binary("target\\release\\igpt.exe");
        }
    } else {
        // 执行 Tauri 的构建
        tauri_build::build();
    }
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn strip_binary(path: &str) {
    let output = Command::new("strip")
        .arg(path)
        .output()
        .expect("Failed to execute strip command");

    if !output.status.success() {
        eprintln!("strip command failed: {:?}", output);
    }
}

#[cfg(target_os = "windows")]
fn strip_binary(path: &str) {
    // 使用 `llvm-strip` 或者其他可用的 strip 工具
    let output = Command::new("llvm-strip")
        .arg(path)
        .output()
        .expect("Failed to execute llvm-strip command");

    if !output.status.success() {
        eprintln!("llvm-strip command failed: {:?}", output);
    }
}
