// use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_files as fs;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");

    HttpServer::new(|| 
		    App::new().service(fs::Files::new("/", "./web").index_file("index.html"))
    )
        .bind(("127.0.0.1", 6725))?
        .run()
        .await
}
