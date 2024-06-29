use scraper::{Html, Selector};

pub fn parse_links(html: &str) -> Vec<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("a").unwrap();
    document.select(&selector)
        .filter_map(|element| element.value().attr("href"))
        .map(|href| href.to_string())
        .collect()
}

pub fn parse_images(html: &str) -> Vec<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("img").unwrap();
    document.select(&selector)
        .filter_map(|element| element.value().attr("src"))
        .map(|src| src.to_string())
        .collect()
}
