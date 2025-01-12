use reqwest::blocking::get;
use std::fs::{self, File};
use std::io::{self, Cursor, Read};
use std::path::Path;
use zip::read::ZipArchive;

pub fn read_file(file_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
pub fn download_file(url: &str, output_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let response = get(url)?;
    let mut file = File::create(output_path)?;
    let mut content = Cursor::new(response.bytes()?);
    io::copy(&mut content, &mut file)?;
    Ok(())
}

pub fn unzip_file(zip_path: &Path, extract_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let out_path = extract_path.join(file.sanitized_name());

        if file.name().ends_with('/') {
            fs::create_dir_all(&out_path)?;
        } else {
            if let Some(parent) = out_path.parent() {
                if !parent.exists() {
                    fs::create_dir_all(&parent)?;
                }
            }
            let mut outfile = File::create(&out_path)?;
            io::copy(&mut file, &mut outfile)?;
        }

        // 设置文件权限（如果有）
        #[cfg(unix)]
        {
            use std::fs::Permissions;
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&out_path, Permissions::from_mode(mode))?;
            }
        }
    }

    Ok(())
}
