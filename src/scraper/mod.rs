use reqwest::Client;

pub async fn test() -> Result<std::string::String, reqwest::Error> {
    let client = reqwest::Client::new();

    let response = client
        .get("https://jsonapi.org/articles")
        .send()
        .await
	.unwrap()
	.text()
	.await;

    return response;

}
