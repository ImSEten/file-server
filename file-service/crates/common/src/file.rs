use std::path::Path;

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
