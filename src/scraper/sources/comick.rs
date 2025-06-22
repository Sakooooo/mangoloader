use crate::scraper::SearchResult;
use reqwest::Client;

pub async fn search(client: reqwest::Client, query: String) {
    println!("{}", query);
}
