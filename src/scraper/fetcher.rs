use reqwest::Client;
use std::error::Error;

pub async fn fetch_url(client: &Client, url: &str) -> Result<String, Box<dyn Error>> {
    let response = client.get(url).send().await?.text().await?;
    Ok(response)
}

pub async fn fetch_data(url: &str) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let data = fetch_url(&client, url).await?;
    Ok(data)
}
