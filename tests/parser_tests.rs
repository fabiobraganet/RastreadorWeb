#[cfg(test)]
mod tests {
    use rastreadorweb::scraper::parser::parse_links;
    use rastreadorweb::scraper::parser::parse_images;
    
    #[test]
    fn test_parse_links() {
        let html = r#"<html><body><a href="http://example.com">Example</a></body></html>"#;
        let links = parse_links(html);
        assert_eq!(links, vec!["http://example.com"]);
    }

    #[test]
    fn test_parse_images() {
        let html = r#"<html><body><img src="http://example.com/image.jpg"></body></html>"#;
        let images = parse_images(html);
        assert_eq!(images, vec!["http://example.com/image.jpg"]);
    }
}
