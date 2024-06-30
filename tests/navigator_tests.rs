use std::collections::HashSet;
use reqwest::Client;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};
use rastreadorweb::scraper::navigator::navigate;

#[tokio::test]
async fn test_navigate() {
    // Start a mock server
    let mock_server = MockServer::start().await;

    // Mock the index.html response
    Mock::given(method("GET"))
        .and(path("/index.html"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_string(
                r#"
                <!DOCTYPE html>
                <html lang="en">
                <head>
                    <meta charset="UTF-8">
                    <meta name="viewport" content="width=device-width, initial-scale=1.0">
                    <title>Site de Teste Local</title>
                </head>
                <body>
                    <h1>Bem-vindo ao Site de Teste Local</h1>
                    <a href="/page1.html">Página 1</a>
                    <a href="/page2.html">Página 2</a>
                    <img src="/image1.jpg" alt="Imagem 1">
                    <img src="/image2.jpg" alt="Imagem 2">
                </body>
                </html>
                "#)
            .insert_header("Content-Type", "text/html")
        )
        .mount(&mock_server)
        .await;

    // Mock the page1.html response
    Mock::given(method("GET"))
        .and(path("/page1.html"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_string(
                r#"
                <!DOCTYPE html>
                <html lang="en">
                <head>
                    <meta charset="UTF-8">
                    <meta name="viewport" content="width=device-width, initial-scale=1.0">
                    <title>Página 1</title>
                </head>
                <body>
                    <h1>Esta é a Página 1</h1>
                    <a href="/index.html">Voltar para a Home</a>
                </body>
                </html>
                "#)
            .insert_header("Content-Type", "text/html")
        )
        .mount(&mock_server)
        .await;

    // Mock the page2.html response
    Mock::given(method("GET"))
        .and(path("/page2.html"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_string(
                r#"
                <!DOCTYPE html>
                <html lang="en">
                <head>
                    <meta charset="UTF-8">
                    <meta name="viewport" content="width=device-width, initial-scale=1.0">
                    <title>Página 2</title>
                </head>
                <body>
                    <h1>Esta é a Página 2</h1>
                    <a href="/index.html">Voltar para a Home</a>
                </body>
                </html>
                "#)
            .insert_header("Content-Type", "text/html")
        )
        .mount(&mock_server)
        .await;

    let client = Client::new();
    let url = &format!("{}/index.html", &mock_server.uri());
    let profundidade = 1;
    let filtro = "links";
    let mut visitados = HashSet::new();
    let dominio = &mock_server.uri().replace("http://", "").replace(":", "_");
    let mut resultados = vec![];
    let formato = "json";
    let output = "output";

    navigate(&client, url, profundidade, filtro, &mut visitados, dominio, formato, output, &mut resultados).await;

    println!("Resultados finais no teste: {:?}", resultados);

    assert!(!resultados.is_empty(), "Nenhum resultado encontrado!");
}
