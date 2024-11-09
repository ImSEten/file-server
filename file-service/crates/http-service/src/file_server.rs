use std::path::PathBuf;

use actix_files::NamedFile;
use actix_web::{get, App, HttpServer, Responder};

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
    //todo! path
    let index_html = PathBuf::from("Source Abs path");

    NamedFile::open_async(index_html).await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .workers(4)
        .run()
        .await
}
