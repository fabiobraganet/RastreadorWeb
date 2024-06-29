use tokio::runtime::Runtime;
use reqwest::Client;
use rastreadorweb::scraper::fetcher::fetch_url;

async fn fetch_data(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let data = fetch_url(&client, url).await?;
    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_url() {
        let rt = Runtime::new().unwrap();
        let result = rt.block_on(fetch_data("http://example.com")).unwrap();
        assert!(result.contains("<title>Example Domain</title>"));
    }
}
