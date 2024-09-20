use axum::{
    routing::get,
    Router,
    response::Html
};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(test));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Ready!");

    axum::serve(listener, app).await.unwrap();
}

async  fn  test() -> Html<&'static str> {
    Html("<h1>Hello</h1>")
}
