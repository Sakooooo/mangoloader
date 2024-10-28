use axum::{Router, routing::get, response::Redirect};
use tower_http::{
    services::ServeDir,
    trace::TraceLayer
};
use crate::api;

pub fn make_routes() -> Router {
        tracing::info!("Mangoloader ready!");
    Router::new()
        .layer(TraceLayer::new_for_http())
	.nest_service("/web", ServeDir::new("./web"))
	.route("/api/test", get(api::json_test))
	.route("/api/version", get(api::get_version))
	// redirect / to /web
	.route("/", get(|| async { Redirect::permanent("/web") }))
}
