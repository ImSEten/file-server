use actix_web::{web, HttpResponse, Responder};
use axum::{
    self,
    body::Body,
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::os::unix::fs::MetadataExt;
use tokio::io::AsyncReadExt;

static INDEX_AXUM_HTML: &str = include_str!("sources/html/index_axum.html");
static INDEX_ACTIX_HTML: &str = include_str!("sources/html/index_actix.html");

#[derive(Serialize, Deserialize, Default)]
struct FileInfo {
    id: u64,
    name: String,
    path: String,
    is_dir: bool,
}

impl FileInfo {
    fn get_file_info(id: u64, path: &std::path::Path) -> Result<Self, std::io::Error> {
        let name = common::file::get_file_name(path)?;
        let is_dir = path.is_dir();
        Ok(FileInfo {
            id,
            name,
            path: common::file::path_to_string(path)?,
            is_dir,
        })
    }
}
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
        let mut id: u64 = 1;
        while let Some(entry) = read_dir.next_entry().await.unwrap_or_default() {
            let path = entry.path();
            if let Ok(file_info) =
                FileInfo::get_file_info(id, &path).map_err(|e| (StatusCode::OK, e.to_string()))
            {
                file_list.push(file_info);
            }
            id += 1;
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
    let mut id: u64 = 1;
    while let Some(entry) = read_dir
        .next_entry()
        .await
        .map_err(|e| (StatusCode::OK, e.to_string()))?
    {
        let path = entry.path();
        file_list
            .push(FileInfo::get_file_info(id, &path).map_err(|e| (StatusCode::OK, e.to_string()))?);
        id += 1;
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
    let mut f = tokio::fs::OpenOptions::new()
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

    let (sender, receiver) = tokio::sync::mpsc::channel::<Result<Vec<u8>, std::io::Error>>(1);
    let stream = tokio_stream::wrappers::ReceiverStream::new(receiver);
    tokio::spawn(async move {
        loop {
            let mut content: Vec<u8> = Vec::with_capacity(1024 * 1024);
            if let Ok(lens) = f.read_buf(&mut content).await {
                if lens == 0 {
                    break; //EOF
                }
                match sender
                    .send(Ok(content))
                    .await
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
                {
                    Ok(_) => {}
                    Err(e) => {
                        sender.send(Err(e)).await.unwrap_or_default();
                        // return Err(std::io::Error::other(error));
                    }
                }
            } else {
                // TODO: send error.
                sender
                    .send(Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("read {:?} got error", file_name),
                    )))
                    .await
                    .unwrap_or_default();
                break;
            }
        }
        //Ok(())
    });
    Ok(Body::from_stream(stream))
}
