pub mod links;
pub mod images;
pub mod common;

use reqwest::Client;
use std::collections::HashSet;

pub async fn navigate(
    client: &Client,
    url: &str,
    profundidade: usize,
    filtro: &str,
    visitados: &mut HashSet<String>,
    dominio: &str,
    formato: &str,
    output: &str,
    resultados: &mut Vec<String>,
) -> Vec<String> {
    match filtro {
        "links" => links::navigate_links(client, url, profundidade, visitados, dominio, formato, output, resultados).await,
        "imagens" => images::navigate_images(client, url, profundidade, visitados, dominio, formato, output, resultados).await,
        _ => panic!("Filtro de navegação desconhecido!"),
    }
}
