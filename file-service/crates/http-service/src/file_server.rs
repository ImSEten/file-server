use actix_web::{get, HttpResponse, Responder};

static INDEX_HTML: &str = include_str!("sources/html/index.html");

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
// #[get("list")]
// async fn list() -> impl Responder{
//     //todo get dir_path from input paramters , now is temp
//     todo!()

// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| App::new().service(index))
//         .bind("127.0.0.1:8080")?
//         .workers(4)
//         .run()
//         .await
// }

// async fn start_http_server() {

// }
