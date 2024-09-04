// use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::{App, HttpServer};
use actix_files as fs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(fs::Files::new("/web", "./web").show_files_listing()))
        .bind(("127.0.0.1", 6725))?
        .run()
        .await
}
