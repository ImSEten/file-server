use actix_web::{get, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{fs, path::Path};

static INDEX_HTML: &str = include_str!("sources/html/index.html");

#[derive(Serialize, Deserialize)]
struct FileInfo {
    id: i64,
    name: String,
    path: String,
    is_dir: bool,
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
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(INDEX_HTML)
}

//list file in dir
#[get("/list")]
async fn list_file() -> impl Responder {
    let root_path = Path::new("/");

    if !root_path.exists() || !root_path.is_dir() {
        return HttpResponse::NotFound()
            .json(json!({"error": "Path not found or not a directory"}));
    }

    let mut file_list = Vec::new();
    let mut id = 0;
    if let Ok(entries) = fs::read_dir(root_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            let name = path.file_name().unwrap().to_str().unwrap().to_string();
            let is_dir = path.is_dir();

            file_list.push(FileInfo {
                id,
                name,
                path: path.to_str().unwrap().to_string(),
                is_dir,
            });
            id += 1;
        }
    }

    HttpResponse::Ok().json(file_list)
}
