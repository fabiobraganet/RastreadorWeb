#[cfg(test)]
mod tests {
    use rastreadorweb::scraper::fetcher::fetch_url;
    use reqwest::Client;

    #[tokio::test]
    async fn test_fetch_url() {
        let client = Client::new();
        let result = fetch_url(&client, "http://example.com").await;
        assert!(result.is_ok());
        let (data, content_type) = result.unwrap();
        assert!(data.contains("<html"));  // Simples verificação para garantir que o HTML foi buscado
        assert!(content_type.contains("text/html")); // Verifica se o content-type é HTML
    }
}
