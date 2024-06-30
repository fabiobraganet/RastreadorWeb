use reqwest::Client;
use std::collections::HashSet;
use url::Url;
use futures::future::join_all;
use crate::scraper::parser::{parse_images, parse_links};
use crate::scraper::fetcher::fetch_url;
use crate::storage::persistence::salvar_resultados; // Certifique-se de que está importando corretamente

/// Navega recursivamente em páginas da web para coletar imagens.
pub async fn navigate_images(
    client: &Client,
    url: &str,
    profundidade: usize,
    visitados: &mut HashSet<String>,
    dominio: &str,
    formato: &str,
    output: &str,
    resultados: &mut Vec<String>,
) -> Vec<String> {
    if profundidade == 0 || visitados.contains(url) {
        println!("Profundidade é 0 ou URL já visitada: {}", url);
        return vec![];
    }
    visitados.insert(url.to_string());

    println!("Navegando na URL: {} com profundidade: {}", url, profundidade);

    match fetch_url(client, url).await {
        Ok((html, content_type)) => process_url_response(
            client,
            url,
            profundidade,
            visitados,
            dominio,
            formato,
            output,
            resultados,
            html,
            content_type,
        ).await,
        Err(e) => {
            println!("Erro ao buscar URL {}: {}", url, e);
            vec![]
        },
    }
}

/// Processa a resposta de uma URL, extraindo e salvando imagens e links.
async fn process_url_response(
    client: &Client,
    url: &str,
    profundidade: usize,
    visitados: &mut HashSet<String>,
    dominio: &str,
    formato: &str,
    output: &str,
    resultados: &mut Vec<String>,
    html: String,
    content_type: String,
) -> Vec<String> {
    if !crate::storage::is_allowed_content_type(&content_type) {
        println!("Tipo de conteúdo não permitido: {}", content_type);
        return vec![];
    }

    let links = parse_links(&html);
    println!("Links encontrados: {:?}", links);

    let subresultados = process_paths(client, url, profundidade, visitados, dominio, formato, output, resultados, links).await;

    for subresultado in subresultados {
        resultados.extend(subresultado);
    }

    println!("Resultados acumulados: {:?}", resultados);

    process_images(html.as_str(), resultados, formato, output);

    resultados.clone()
}

/// Processa e salva imagens encontradas em uma página HTML.
fn process_images(
    html: &str,
    resultados: &mut Vec<String>,
    formato: &str,
    output: &str,
) {
    let imagens = parse_images(html);
    println!("Imagens encontradas: {:?}", imagens);
    resultados.extend(imagens);

    if resultados.len() >= 1000 {
        salvar_resultados(resultados, formato, output);
        resultados.clear();
    }
}

/// Processa os caminhos (links) encontrados em uma página HTML, seguindo-os recursivamente.
async fn process_paths(
    client: &Client,
    url: &str,
    profundidade: usize,
    visitados: &mut HashSet<String>,
    dominio: &str,
    formato: &str,
    output: &str,
    resultados: &mut Vec<String>,
    links: Vec<String>,
) -> Vec<Vec<String>> {
    let mut tarefas = vec![];

    for link in links {
        if let Ok(url_absoluta) = Url::parse(url).unwrap().join(&link) {
            if !visitados.contains(url_absoluta.as_str()) {
                if url_absoluta.domain() == Some(dominio) {
                    println!("Seguindo link: {}", url_absoluta);
                    let client_clone = client.clone();
                    let url_absoluta_clone = url_absoluta.to_string();
                    let mut visitados_clone = visitados.clone();
                    let mut resultados_clone = resultados.clone();
                    tarefas.push(async move {
                        navigate_images(&client_clone, &url_absoluta_clone, profundidade - 1, &mut visitados_clone, dominio, formato, output, &mut resultados_clone).await
                    });
                } else {
                    println!("URL fora do domínio: {}", url_absoluta);
                }
            } else {
                println!("URL já visitada: {}", url_absoluta);
            }
        } else {
            println!("Erro ao parsear URL: {}", link);
        }
    }

    join_all(tarefas).await
}
