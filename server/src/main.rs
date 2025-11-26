use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
//use actix_files::NamedFile;
//use std::path::PathBuf;
use actix_files as fs;



#[derive(Serialize, Deserialize)]
struct Item {
    id: u32,
    name: String,
}
/* 
#[get("/page")]
async fn serve_page() -> impl Responder {
    let path: PathBuf = PathBuf::from("static/index.html");
    NamedFile::open(path)
}
*/

// GET endpoint: simple health check
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Mini REST server is running 🚀")
}

// GET endpoint: return JSON
#[get("/item/{id}")]
async fn get_item(path: web::Path<u32>) -> impl Responder {
    let item = Item {
        id: path.into_inner(),
        name: "Sample Item".to_string(),
    };
    HttpResponse::Ok().json(item)
}

// POST endpoint: accept JSON
#[post("/item")]
async fn create_item(item: web::Json<Item>) -> impl Responder {
    HttpResponse::Created().json(item.into_inner())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .service(fs::Files::new("/asm", "../asm/pkg").show_files_listing())
            //.service(serve_page)
            .service(hello)
            .service(get_item)
            .service(create_item)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
