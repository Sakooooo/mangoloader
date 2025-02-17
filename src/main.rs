mod api;

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::fmt()
        // Use a more compact, abbreviated log format
        .compact()
        // Display source code file paths
        .with_file(true)
        // Display source code line numbers
        .with_line_number(true)
        // Don't display the event's target (module path)
        .with_target(false)
        // Build the subscriber
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

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, api::web::serve().await)
        .await
        .unwrap();
}
