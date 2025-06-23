use hyper::header::{CONTENT_TYPE, USER_AGENT};
use reqwest::header::HeaderMap;
use serde::Deserialize;
use std::{error::Error, fmt};

// this'll just query comick for now
// we'll probably want to seperate this later
#[derive(Debug)]
enum ComicKError {
    RequestFailed(reqwest::Error),
    JsonError(serde_json::Error),
}

impl fmt::Display for ComicKError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RequestFailed(e) => write!(f, "Failed to make request: {}", e),
            Self::JsonError(e) => write!(f, "Failed to parse json: {}", e),
        }
    }
}

impl From<reqwest::Error> for ComicKError {
    fn from(err: reqwest::Error) -> Self {
        Self::RequestFailed(err)
    }
}

impl From<serde_json::Error> for ComicKError {
    fn from(err: serde_json::Error) -> Self {
        Self::JsonError(err)
    }
}

#[derive(Debug, Deserialize)]
struct Search {
    id: u32,
    slug: String,
    title: String,
    desc: Option<String>,
}

#[derive(Debug, Deserialize)]
struct FirstChapter {
    chapter: String,
    hid: String,
    // todo maybe make this an enum
    lang: String,
    volume: String,
}

#[derive(Debug, Deserialize)]
struct ComicInfo {
    id: u32,
    hid: String,
    title: String,
    country: String,
    status: u16, // i need to make an enum for this i don't know why its specified as numbers lol
    last_chapter: u32,
    chapter_count: u64,
    desc: String,
}

#[derive(Debug, Deserialize)]
struct Info {}

async fn search(client: reqwest::Client, query: String) -> Result<Vec<Search>, ComicKError> {
    let mut headers = HeaderMap::new();
    headers.insert(
        USER_AGENT,
        "Mozilla/5.0 (compatible; PHP script)".parse().unwrap(),
    );

    let response: Vec<Search> = client
        .get(format!(
            "https://api.comick.fun/v1.0/search/?tachiyomi=true&q={}",
            query
        ))
        .headers(headers)
        .send()
        .await?
        .json::<Vec<Search>>()
        .await?;

    Ok(response)
}

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();

    // use example for now
    let search: Vec<Search> = search(client, "jujitsu".to_string()).await.unwrap();
    println!("{:?}", search);
}
