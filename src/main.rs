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
use sqlx::{database, migrate::MigrateDatabase, Sqlite, SqlitePool};
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(long)]
    datadir: String,
}

#[derive(Serialize)]
struct Manga {
    id: u64,
    name: String,
    link: String, 
    status: String,
    chapters: u64,
    chapters_downloaded: u64,
    path: String,
}

#[derive(Serialize)]
struct Hello{
    test: String,
}

#[tokio::main]
async fn main(){

    let args = Args::parse();

    // TODO(sako) make this a commandline option maybe..
    // const DATABASE: &str = "sqlite://./config/database.db";

    // what
    // todo check if dir exists and make it
    let data_dir: String = args.datadir.to_owned();
    let database_file: &str = "/database.db";
    let what = data_dir + database_file;
    // i hate this
    let database: &str = what.as_str();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    tracing::info!("Preparing database drivers...");
    sqlx::any::install_default_drivers();

    if !Sqlite::database_exists(database).await.unwrap_or(false) {
	tracing::info!("Creating database at {}...", database);
        match Sqlite::create_database(database).await {
	    Ok(_) => tracing::info!("Database created successfully!"),
	    Err(error) => panic!("error: {}", error), 
        }
    } else {
	tracing::info!("Database already exists...");
    }

    let db = SqlitePool::connect(database).await.unwrap();
    let result = sqlx::query("CREATE TABLE IF NOT EXISTS manga (id INTEGER PRIMARY KEY NOT NULL, name ntext NOT NULL, link text NOT NULL, status text NOT NULL, chapters int NOT NULL, chapters_downloaded int NOT NULL);").execute(&db).await.unwrap();
    tracing::info!("Created manga table: {:?}", result);

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
