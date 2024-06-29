#[cfg(test)]
mod tests {
    use reqwest::Client;
    use std::collections::HashSet;
    use tokio::runtime::Runtime;
    use rastreadorweb::scraper::navigator::navigate;

    #[test]
    fn test_navigate() {
        let rt = Runtime::new().unwrap();
        let client = Client::new();
        let mut visitados = HashSet::new();
        let result = rt.block_on(navigate(&client, "http://example.com", 1, "links", &mut visitados));
        assert!(result.len() > 0);
    }
}
