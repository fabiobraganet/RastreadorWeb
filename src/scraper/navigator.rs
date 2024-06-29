use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::HashSet;
use url::Url;
use futures::future::join_all;

pub async fn fetch_url(client: &Client, url: &str) -> Result<String, reqwest::Error> {
    let response = client.get(url).send().await?.text().await?;
    Ok(response)
}

pub async fn navigate(
    client: &Client,
    url: &str,
    profundidade: usize,
    filtro: &str,
    visitados: &mut HashSet<String>,
) -> Vec<String> {
    if profundidade == 0 || visitados.contains(url) {
        return vec![];
    }
    visitados.insert(url.to_string());
    
    match fetch_url(client, url).await {
        Ok(html) => {
            let documento = Html::parse_document(&html);
            let seletor = match filtro {
                "imagens" => Selector::parse("img").unwrap(),
                _ => Selector::parse("a").unwrap(),
            };

            let elementos: Vec<String> = documento
                .select(&seletor)
                .filter_map(|element| match filtro {
                    "imagens" => element.value().attr("src").map(|s| s.to_string()),
                    _ => element.value().attr("href").map(|s| s.to_string()),
                })
                .collect();

            let mut tarefas = vec![];
            for link in &elementos {
                if let Ok(url_absoluta) = Url::parse(url).unwrap().join(link) {
                    let client_clone = client.clone();
                    let url_absoluta_clone = url_absoluta.to_string();
                    let mut visitados_clone = visitados.clone();
                    tarefas.push(async move {
                        navigate(&client_clone, &url_absoluta_clone, profundidade - 1, filtro, &mut visitados_clone).await
                    });
                }
            }

            let resultados: Vec<Vec<String>> = join_all(tarefas).await;
            elementos.into_iter().chain(resultados.into_iter().flatten()).collect()
        }
        Err(_) => vec![],
    }
}
