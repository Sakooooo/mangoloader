use axum::{
    routing::get,
    Router,
    // http::StatusCode,
    response::Redirect,
};
use std::net::SocketAddr;
// use tower::ServiceExt;
use tower_http::{
    services::ServeDir,
    trace::TraceLayer,
};
use tracing_subscriber;

#[tokio::main]
async fn main(){

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

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

fn begin_serve() -> Router {
    // serve the file in the "web" directory under `/web`
    tracing::info!("Mangoloader ready!");
    Router::new()
        .layer(TraceLayer::new_for_http())
	.nest_service("/web", ServeDir::new("./web"))
	.route("/api/test", get(|| async {"Test"}))
	// redirect / to /web
	.route("/", get(|| async { Redirect::permanent("/web") }))
}
