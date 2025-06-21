use std::net::ToSocketAddrs;

mod api;
mod scraper;

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .with_max_level(if cfg!(debug_assertions) {
            tracing::Level::DEBUG
        } else {
            tracing::Level::INFO
        })
        .finish();

    match tracing::subscriber::set_global_default(subscriber) {
        Ok(_) => {}
        Err(err) => {
            tracing::error!("Could not create logger!");
            panic!("{}", err);
        }
    };

    let address = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    tracing::info!("listening at {}", address);

    axum::serve(listener, api::web::serve().await)
        .await
        .unwrap();
}
