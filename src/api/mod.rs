pub mod manga;
pub mod routes;

use serde::Serialize;
use axum::Json;

#[derive(Serialize)]
struct Hello{
    test: String,
}

#[derive(Serialize)]
struct Version {
    version: String,
    
}
const VERSION: &str = env!("CARGO_PKG_VERSION");

async fn get_version() -> Json<Vec<Version>> {
    let version = vec![
	Version {
	    version: VERSION.to_string(),
	}
    ];

    Json(version)
}

async fn json_test() -> Json<Vec<Hello>> {
    let hello = vec![
	Hello {
	    test: "Hello Rust".to_string(),
	}
    ];

    Json(hello)
}

