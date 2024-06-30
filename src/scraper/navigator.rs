use reqwest::Client;
use std::collections::HashSet;
use url::Url;
use futures::future::join_all;
use crate::scraper::parser::{parse_links, parse_images};
use crate::scraper::fetcher::fetch_url;
use crate::storage::{is_allowed_content_type, csv::save_as_csv, json::save_as_json};

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
    if profundidade == 0 || visitados.contains(url) {
        println!("Profundidade é 0 ou URL já visitada: {}", url);
        return vec![];
    }
    visitados.insert(url.to_string());

    println!("Navegando na URL: {} com profundidade: {}", url, profundidade);

    match fetch_url(client, url).await {
        Ok((html, content_type)) => {
            if !is_allowed_content_type(&content_type) {
                println!("Tipo de conteúdo não permitido: {}", content_type);
                return vec![];
            }

            if filtro == "imagens" {
                let imagens = parse_images(&html);
                println!("Imagens encontradas: {:?}", imagens);
                resultados.extend(imagens);
                if resultados.len() >= 1000 {
                    salvar_resultados(resultados, formato, output);
                }
            }

            let links = parse_links(&html);
            println!("Links encontrados: {:?}", links);
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
                                navigate(&client_clone, &url_absoluta_clone, profundidade - 1, filtro, &mut visitados_clone, dominio, formato, output, &mut resultados_clone).await
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

            let subresultados: Vec<Vec<String>> = join_all(tarefas).await;
            for subresultado in subresultados {
                resultados.extend(subresultado);
            }

            println!("Resultados acumulados: {:?}", resultados);

            salvar_resultados(resultados, formato, output);

            resultados.clone()
        }
        Err(e) => {
            println!("Erro ao buscar URL {}: {}", url, e);
            vec![]
        },
    }
}

fn salvar_resultados(resultados: &mut Vec<String>, formato: &str, output: &str) {
    match formato {
        "json" => save_as_json(resultados, output).expect("Erro ao escrever resultados no arquivo JSON"),
        "csv" => save_as_csv(resultados, output).expect("Erro ao escrever resultados no arquivo CSV"),
        _ => panic!("Formato de saída não suportado!"),
    }
    resultados.clear();
}
