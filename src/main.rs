use axum::{
    routing::get,
    Router,
    response::Html,
    // http::StatusCode,
};
use std::net::SocketAddr;
// use tower::ServiceExt;
use tower_http::{
    services::ServeDir,
    trace::TraceLayer,
};
// use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // // build our application with a single route
    // let app = Router::new().route("/web", get(test));
    // // run our app with hyper, listening globally on port 3000
    // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // println!("Ready!");

    // axum::serve(listener, app).await.unwrap();

    tokio::join!(
        serve(begin_serve(), 3000),
    );

    println!("Mangoloader ready!");
}

async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    // tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.layer(TraceLayer::new_for_http()))
        .await
        .unwrap();
}

fn begin_serve() -> Router {
    // serve the file in the "web" directory under `/web`
    Router::new()
	.nest_service("/web", ServeDir::new("./web"))
	.route("/api/test", get(|| async {"Test"}))
}
