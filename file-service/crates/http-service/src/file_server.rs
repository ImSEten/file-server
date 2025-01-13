use actix_web::{web, HttpResponse, Responder};
use axum::{
    self,
    extract::{Multipart, Path},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    Json,
};
use common::file::FileInfo;
use serde_json::json;
use std::{os::unix::fs::MetadataExt, path::PathBuf};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

static INDEX_AXUM_HTML: &str = include_str!("sources/html/index_axum.html");
static INDEX_ACTIX_HTML: &str = include_str!("sources/html/index_actix.html");

//default uploads dir , todo! get it from config
// const UPLOAD_DIR: &str = "uploads";

// TODO
// #[post("/upload")]
// async fn upload(mut payload: Multipart) -> impl Responder {
//    todo!()
// }

//TODO
// #[get("/download/{filename}")]
// async fn download(filename: web::Path<String>) -> impl Responder {
//    todo!()
// }

// TODO
pub async fn index_actix() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(INDEX_ACTIX_HTML)
}

//list file in dir
pub async fn list_actix(path: web::Path<String>) -> impl Responder {
    log::debug!("list request for directory = {:?}", path);
    let mut path = path.to_string();
    if !path.starts_with("/") {
        path = format!("/{}", path);
    }
    let root_path = std::path::Path::new(&path);

    let mut file_list = Vec::<FileInfo>::new();
    if !root_path.exists() || !root_path.is_dir() {
        return HttpResponse::NotFound()
            .json(json!({"error": "Path not found or not a directory"}));
    }

    if let Ok(mut read_dir) = tokio::fs::read_dir(root_path)
        .await
        .map_err(|e| (StatusCode::OK, e.to_string()))
    {
        while let Some(entry) = read_dir.next_entry().await.unwrap_or_default() {
            let path = entry.path();
            if let Ok(file_info) = FileInfo::new(&path)
                .await
                .map_err(|e| (StatusCode::OK, e.to_string()))
            {
                file_list.push(file_info);
            }
        }
    }
    HttpResponse::Ok().json(file_list)
}

// pub async fn download_file_actix() -> impl Responder {

// }

pub async fn not_found_axum() -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, "404 Not Found".to_string())
}

pub async fn index_axum() -> impl IntoResponse {
    Html(INDEX_AXUM_HTML)
}

//list file in dir
pub async fn list_axum(
    Path(mut directory): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    log::debug!("list request for directory = {:?}", directory);
    if !directory.starts_with("/") {
        directory = format!("/{}", directory);
    }
    let root_path = std::path::Path::new(&directory);

    let mut file_list = Vec::<FileInfo>::new();
    if !root_path.exists() || !root_path.is_dir() {
        return Err((StatusCode::NOT_FOUND, "path not exist".to_string()));
    }

    let mut read_dir = tokio::fs::read_dir(root_path)
        .await
        .map_err(|e| (StatusCode::OK, e.to_string()))?;
    while let Some(entry) = read_dir
        .next_entry()
        .await
        .map_err(|e| (StatusCode::OK, e.to_string()))?
    {
        let path = entry.path();
        file_list.push(
            FileInfo::new(&path)
                .await
                .map_err(|e| (StatusCode::OK, e.to_string()))?,
        );
    }

    Ok(Json(file_list))
}

pub async fn download_file_axum(
    Path(mut file_name): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    log::debug!("download_file request for file_name = {:?}", file_name);
    if !file_name.starts_with("/") {
        file_name = format!("/{}", file_name);
    }
    let file = std::path::Path::new(&file_name);
    if file.is_dir() {
        return Err((StatusCode::OK, "file if dir, cannot download".to_string()));
    }
    let f = tokio::fs::OpenOptions::new()
        .read(true)
        .open(file)
        .await
        .unwrap();
    let _mode = f
        .metadata()
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
        .unwrap()
        .mode();

    let name = file
        .file_name()
        .and_then(|f| f.to_str())
        .unwrap_or("unknown");
    let content_disposition = format!("attachment; filename=\"{}\"", name);
    let receiver = common::file::read_file_content(file_name).await.unwrap();
    let stream = tokio_stream::wrappers::ReceiverStream::new(receiver);
    match axum::response::Response::builder()
        .status(StatusCode::OK)
        .header(axum::http::header::CONTENT_DISPOSITION, content_disposition)
        .header(axum::http::header::CONTENT_TYPE, "application/octet-stream")
        .body(axum::body::Body::from_stream(stream))
    {
        Ok(body) => Ok(body),
        Err(e) => Err((StatusCode::OK, e.to_string())),
    }
}

// TODO: upload_file by block.
pub async fn upload_file_axum(
    Path(mut directory): Path<String>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    log::debug!("upload_file request for directory = {:?}", directory);
    // Ensure the directory starts with a slash if not already present
    if !directory.starts_with('/') {
        directory = format!("/{}", directory);
    }
    // Create the directory if it doesn't exist
    let dir_path = PathBuf::from(&directory);
    if !dir_path.exists() {
        tokio::fs::create_dir_all(&dir_path).await.map_err(|e| {
            log::error!("create directory {:?} error {}", dir_path, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to create directory: {}", e),
            )
        })?;
    }

    // TODO: every field is a file, so we need to use tokio::spawn to deal with every file.
    while let Some(field) = multipart.next_field().await.map_err(|e| {
        log::error!("multipart.next_field error: {}", e);
        (
            StatusCode::BAD_REQUEST,
            format!("Failed to get field: {}", e),
        )
    })? {
        let filename = field.file_name().unwrap_or("unknown");
        let filepath = dir_path.join(filename);
        // Check if file already exists and handle accordingly
        if filepath.exists() {
            log::error!("file {:?} exists", filepath);
            return Err((StatusCode::CONFLICT, "File already exists".to_string()));
        }
        // while let Some(chunk) = field.chunk().await.map_err(|e|
        //     {
        //     log::error!("field.chunk error: {}", e);
        //     (StatusCode::BAD_REQUEST, e.to_string())
        //     })? {
        // }
        let data = field.bytes().await.map_err(|e| {
            log::error!("bytes get error: {}", e);
            (
                StatusCode::BAD_REQUEST,
                format!("Failed to read file data: {}", e),
            )
        })?;
        let mut f = tokio::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(filepath)
            .await
            .map_err(|e| {
                log::error!("open file error: {}", e);
                (StatusCode::NOT_FOUND, e.to_string())
            })?;
        f.write_all(&data).await.map_err(|e| {
            log::error!("write failed: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to write file to disk: {}", e),
            )
        })?;
    }
    // log::info!("File uploaded successfully: {:?}", filepath);
    Ok(Response::builder()
        .status(StatusCode::CREATED)
        .header("Location", "filepath")
        .body("File uploaded successfully.".to_string())
        .unwrap())
}

// delete file in dir
pub async fn delete_file_axum(
    Path(mut directory): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    log::debug!("delete request for directory = {:?}", directory);
    if !directory.starts_with("/") {
        directory = format!("/{}", directory);
    }
    let file_path = std::path::Path::new(&directory);

    if !file_path.exists() {
        return Err((StatusCode::NOT_FOUND, "path not exist".to_string()));
    }

    if file_path.is_dir() {
        match tokio::fs::remove_dir_all(file_path).await {
            Ok(_) => Ok(Json("delete finished")),
            Err(e) => Err((StatusCode::NOT_FOUND, e.to_string())),
        }
    } else {
        match tokio::fs::remove_file(file_path).await {
            Ok(_) => Ok(Json("delete finished")),
            Err(e) => Err((StatusCode::NOT_FOUND, e.to_string())),
        }
    }
}

//list file in dir
pub async fn merge_file_axum(
    Path(mut directory): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    log::debug!("merge request for directory = {:?}", directory);
    if !directory.starts_with("/") {
        directory = format!("/{}", directory);
    }
    let parent = common::file::get_file_parent(std::path::Path::new(&directory)).map_err(|e| {
        log::error!("get_file_parent {} error: {}", &directory, e);
        (StatusCode::NOT_FOUND, e.to_string())
    })?;
    let file_real_name =
        common::file::get_file_name(std::path::Path::new(&directory)).map_err(|e| {
            log::error!("get_file_name {} error: {}", &directory, e);
            (StatusCode::NOT_FOUND, e.to_string())
        })?;

    let mut read_dir = tokio::fs::read_dir(&parent).await.map_err(|e| {
        log::error!("read_dir {} error: {}", parent, e);
        (StatusCode::OK, e.to_string())
    })?;
    let mut file_list = Vec::<(u32, String)>::new();
    while let Some(entry) = read_dir.next_entry().await.map_err(|e| {
        log::error!("read_dir next_entry error: {}", e);
        (StatusCode::OK, e.to_string())
    })? {
        let path = entry.path();
        let file_name = common::file::get_file_name(&path).map_err(|e| {
            log::error!("get_file_name {:?} error: {}", &path, e);
            (StatusCode::NOT_FOUND, e.to_string())
        })?;
        if let Some(i) = is_suffix_with_number(&file_real_name, &file_name) {
            file_list.push((i, file_name));
        }
    }
    file_list.sort_by_key(|k| k.0);
    let mut f = tokio::fs::OpenOptions::new()
        .append(true)
        .open(&directory)
        .await
        .map_err(|e| {
            log::error!("open file {} error: {}", directory, e);
            (StatusCode::NOT_FOUND, e.to_string())
        })?;
    for (_, tmp_file) in file_list {
        let mut f_tmp = tokio::fs::OpenOptions::new()
            .read(true)
            .open(std::path::Path::new(&parent).join(&tmp_file))
            .await
            .map_err(|e| {
                log::error!("open file {} error: {}", tmp_file, e);
                (StatusCode::NOT_FOUND, e.to_string())
            })?;
        let mut buf_tmp: Vec<u8> = Vec::new();
        f_tmp.read_to_end(&mut buf_tmp).await.map_err(|e| {
            log::error!("read file {} error: {}", &tmp_file, e);
            (StatusCode::NOT_FOUND, e.to_string())
        })?;
        f.write_all(&buf_tmp).await.map_err(|e| {
            log::error!("write file {} error: {}", &directory, e);
            (StatusCode::NOT_FOUND, e.to_string())
        })?;
        tokio::fs::remove_file(std::path::Path::new(&parent).join(&tmp_file))
            .await
            .map_err(|e| {
                log::error!("remove file {} error: {}", &tmp_file, e);
                (StatusCode::NOT_FOUND, e.to_string())
            })
            .unwrap_or_default();
    }
    Ok(Json("Merge finished"))
}

fn is_suffix_with_number(original: &str, candidate: &str) -> Option<u32> {
    // 检查candidate是否以original开头，并且紧接着是一个点
    if !candidate.starts_with(original) || original.len() >= candidate.len() {
        return None;
    }

    // 获取original之后的部分
    let suffix = &candidate[original.len()..];

    // 检查suffix是否以'.'开始
    if !suffix.starts_with('.') {
        return None;
    }

    // 获取'.‘之后的数字部分
    let number_part = &suffix[1..];

    // 尝试将number_part解析为一个整数，如果成功，则返回true
    number_part.parse::<u32>().ok()
}
