mod cli;
mod scraper;
mod storage;

use cli::build_cli;
use scraper::navigator::navigate;
use std::collections::HashSet;
use tokio::runtime::Runtime;
use reqwest::Client;

fn main() {
    let matches = build_cli().get_matches();

    let url = matches.value_of("url").unwrap();
    let profundidade = matches.value_of("profundidade").unwrap_or("1").parse::<usize>().unwrap();
    let filtro = matches.value_of("filtro").unwrap_or("links");
    let formato = matches.value_of("formato").unwrap_or("json");
    let output = matches.value_of("output").unwrap_or("output");

    let dominio = url::Url::parse(url).unwrap().host_str().unwrap().to_string();

    let rt = Runtime::new().unwrap();
    let client = Client::new();
    let mut visitados = HashSet::new();
    let mut resultados = vec![];

    rt.block_on(navigate(&client, url, profundidade, filtro, &mut visitados, &dominio, formato, output, &mut resultados));
}
