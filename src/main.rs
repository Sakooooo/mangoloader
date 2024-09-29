use axum::{
    routing::get,
    Router,
    Json,
    // http::StatusCode,
    response::Redirect,
};
use std::{net::SocketAddr, process};
// use tower::ServiceExt;
use tower_http::{
    services::ServeDir,
    trace::TraceLayer,
};
use tracing_subscriber;
use serde::Serialize;
use sqlx::{self, sqlite};

#[derive(Serialize)]
struct Manga {
    id: u64,
    name: String,
    link: String, 
    status: String,
    chapters: u64,
    chapters_downloaded: u64,
}

#[derive(Serialize)]
struct Hello{
    test: String,
}

#[tokio::main]
async fn main(){

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    tracing::info!("Preparing database...");
    sqlx::any::install_default_drivers();
    let db = sqlx::sqlite::SqlitePool::connect("sqlite:mydb.db").await;
    tracing::info!("Database ready!");
    tracing::info!("Starting Mangoloader...");
    tokio::join!(
        serve(begin_serve(), 3000),
    );
}

async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.layer(TraceLayer::new_for_http()))
        .await
        .unwrap();
}

async fn json_test() -> Json<Vec<Hello>> {
    let hello = vec![
	Hello {
	    test: "Hello Rust".to_string(),
	}
    ];

    Json(hello)
}

fn begin_serve() -> Router {
    // serve the file in the "web" directory under `/web`
    tracing::info!("Mangoloader ready!");
    Router::new()
        .layer(TraceLayer::new_for_http())
	.nest_service("/web", ServeDir::new("./web"))
	.route("/api/test", get(json_test))
	// redirect / to /web
	.route("/", get(|| async { Redirect::permanent("/web") }))
}
