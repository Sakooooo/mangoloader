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
    add_modal: bool,
}

async fn index(add: bool) -> Html<String> {
    let template = Index { add_modal: add };
    return Html(template.render().unwrap());
}

pub async fn serve() -> Router {
    tracing::info!("creating routes");
    return Router::new()
        .layer(TraceLayer::new_for_http())
        .nest_service("/static", ServeDir::new("static"))
        .route("/", get(index(false).await));
}
