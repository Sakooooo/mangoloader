use askama::Template;
use axum::{
    response::Html,
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

#[derive(Template)]
#[template(path = "index.html")]
struct Index {
    version: String,
    // todo maybe we're supposed to use Option<>?
    sources: Vec<String>,
}

#[derive(Template)]
#[template(path = "add.html")]
struct Add {
    sources: Vec<String>,
}

async fn index() -> Html<String> {
    let dummy_sources: Vec<String> = vec![
        "source1".to_string(),
        "source2".to_string(),
        "source3".to_string(),
    ];
    let template = Index {
        version: env!("CARGO_PKG_VERSION").to_string(),
        sources: dummy_sources,
    };
    return Html(template.render().unwrap());
}

async fn add() -> Html<String> {
    let sources: Vec<String> = vec![
        "source1".to_string(),
        "source2".to_string(),
        "source3".to_string(),
    ];
    let template = Add { sources };
    return Html(template.render().unwrap());
}

pub async fn serve() -> Router {
    tracing::info!("mangoloader ready");
    return Router::new()
        .layer(TraceLayer::new_for_http())
        .nest_service("/static", ServeDir::new("static"))
        .route("/", get(index))
        .route("/add", get(add));
}
