use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Serialize, Deserialize, Default)]
pub struct FileInfo {
    pub size: u64,
    pub name: String,
    pub path: String,
    pub is_dir: bool,
}

impl FileInfo {
    pub async fn new(path: &std::path::Path) -> Result<Self, std::io::Error> {
        let name = get_file_name(path)?;
        let is_dir = path.is_dir();
        let mut size = tokio::fs::metadata(&path).await?.len();
        if is_dir {
            size = 0;
        }
        Ok(FileInfo {
            size,
            name,
            path: path_to_string(path)?,
            is_dir,
        })
    }
}

pub fn path_to_string(file: &Path) -> Result<String, std::io::Error> {
    Ok(file
        .as_os_str()
        .to_str()
        .ok_or(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "file name is incorrect",
        ))?
        .to_string())
}

pub fn get_file_name(file: &Path) -> Result<String, std::io::Error> {
    let file_name = file
        .file_name()
        .ok_or(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "file name is incorrect",
        ))?
        .to_str()
        .ok_or(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "file name contains non-UTF-8 charactors",
        ))?
        .to_string();
    Ok(file_name)
}

pub fn get_file_parent(file: &Path) -> Result<String, std::io::Error> {
    let file_parent = file
        .parent()
        .ok_or(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "file name is incorrect",
        ))?
        .as_os_str()
        .to_str()
        .ok_or(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "file name contains non-UTF-8 charactors",
        ))?
        .to_string();
    Ok(file_parent)
}
