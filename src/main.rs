use std::error::Error;

// this'll just query comick for now
// we'll probably want to seperate this later
struct Search {}

async fn search(client: reqwest::Client, query: String) {
    let response = client
        .get(format!(
            "https://api.comick.fun/v1.0/search/?tachiyomi=true&q={}",
            query
        ))
        .send()
        .await
        .unwrap();

    println!("{:? }", response);
}

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();

    let result = search(client, "test".to_string()).await;
    println!("Hello");
}
