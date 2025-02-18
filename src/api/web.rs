use axum::{
    response::Html,
    routing::{get, post},
    Router,
};
use rinja_axum::Template;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

#[derive(Template)]
#[template(path = "index.html")]
struct Index {
    // todo maybe we're supposed to use Option<>?
    sources: Vec<String>,
}

async fn index() -> Html<String> {
    let dummy_sources: Vec<String> = vec![
        "source1".to_string(),
        "source2".to_string(),
        "source3".to_string(),
    ];
    let template = Index {
        sources: dummy_sources,
    };
    return Html(template.render().unwrap());
}

pub async fn serve() -> Router {
    tracing::info!("creating routes");
    return Router::new()
        .layer(TraceLayer::new_for_http())
        .nest_service("/static", ServeDir::new("static"))
        .route("/", get(index));
}
