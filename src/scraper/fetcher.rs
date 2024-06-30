use reqwest::Client;

pub async fn fetch_url(client: &Client, url: &str) -> Result<(String, String), reqwest::Error> {
    let response = client.get(url).send().await?;
    let content_type = response.headers().get("content-type").map(|v| v.to_str().unwrap_or("")).unwrap_or("").to_string();
    let text = response.text().await?;
    Ok((text, content_type))
}

