// use reqwest::Client;
// use std::collections::HashSet;
// use url::Url;
// use futures::future::join_all;
// use crate::scraper::fetcher::fetch_url;
// use crate::scraper::parser::{parse_links, parse_images};
// use crate::storage::persistence::salvar_resultados;
// use crate::scraper::navigator::{navigate_images, navigate_links};
// use std::pin::Pin;
// use std::future::Future;

// /// Processa e salva imagens encontradas em uma página HTML.
// pub fn process_images(
//     html: &str,
//     resultados: &mut Vec<String>,
//     formato: &str,
//     output: &str,
// ) {
//     let imagens = parse_images(html);
//     println!("Imagens encontradas: {:?}", imagens);
//     resultados.extend(imagens);

//     if resultados.len() >= 1000 {
//         salvar_resultados(resultados, formato, output);
//         resultados.clear();
//     }
// }

// /// Processa os caminhos (links) encontrados em uma página HTML, seguindo-os recursivamente.
// pub async fn process_paths(
//     client: &Client,
//     url: &str,
//     profundidade: usize,
//     visitados: &mut HashSet<String>,
//     dominio: &str,
//     formato: &str,
//     output: &str,
//     resultados: &mut Vec<String>,
//     links: Vec<String>,
//     filtro: &str,
// ) -> Vec<Vec<String>> {
//     let mut tarefas = vec![];

//     for link in links {
//         if let Ok(url_absoluta) = Url::parse(url).unwrap().join(&link) {
//             if !visitados.contains(url_absoluta.as_str()) {
//                 if url_absoluta.domain() == Some(dominio) {
//                     println!("Seguindo link: {}", url_absoluta);
//                     let client_clone = client.clone();
//                     let url_absoluta_clone = url_absoluta.to_string();
//                     let mut visitados_clone = visitados.clone();
//                     let mut resultados_clone = resultados.clone();

//                     let tarefa: Pin<Box<dyn Future<Output = Vec<String>> + Send>> = match filtro {
//                         "links" => {
//                             Box::pin(async move {
//                                 navigate_links(&client_clone, &url_absoluta_clone, profundidade - 1, &mut visitados_clone, dominio, formato, output, &mut resultados_clone).await
//                             })
//                         },
//                         "imagens" => {
//                             Box::pin(async move {
//                                 navigate_images(&client_clone, &url_absoluta_clone, profundidade - 1, &mut visitados_clone, dominio, formato, output, &mut resultados_clone).await
//                             })
//                         },
//                         _ => panic!("Filtro de navegação desconhecido!"),
//                     };

//                     tarefas.push(tarefa);
//                 } else {
//                     println!("URL fora do domínio: {}", url_absoluta);
//                 }
//             } else {
//                 println!("URL já visitada: {}", url_absoluta);
//             }
//         } else {
//             println!("Erro ao parsear URL: {}", link);
//         }
//     }

//     join_all(tarefas).await
// }

// /// Processa a resposta de uma URL, extraindo e salvando imagens e links.
// pub async fn process_url_response(
//     client: &Client,
//     url: &str,
//     profundidade: usize,
//     visitados: &mut HashSet<String>,
//     dominio: &str,
//     formato: &str,
//     output: &str,
//     resultados: &mut Vec<String>,
//     html: String,
//     content_type: String,
//     parse_func: fn(&str) -> Vec<String>,
// ) -> Vec<String> {
//     if !crate::storage::is_allowed_content_type(&content_type) {
//         println!("Tipo de conteúdo não permitido: {}", content_type);
//         return vec![];
//     }

//     let links = parse_func(&html);
//     println!("Links encontrados: {:?}", links);

//     let subresultados = process_paths(client, url, profundidade, visitados, dominio, formato, output, resultados, links, "links").await;

//     for subresultado in subresultados {
//         resultados.extend(subresultado);
//     }

//     println!("Resultados acumulados: {:?}", resultados);

//     process_images(&html, resultados, formato, output);

//     resultados.clone()
// }
