use actix_web::{web, HttpResponse, Responder};
use axum::{
    self,
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
    Json,
};
use common::file::FileInfo;
use serde_json::json;
use std::os::unix::fs::MetadataExt;

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
