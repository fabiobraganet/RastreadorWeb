mod cli;

use cli::build_cli;

fn main() {
    let matches = build_cli().get_matches();

    let url = matches.value_of("url").unwrap();
    let profundidade = matches.value_of("profundidade").unwrap_or("1").parse::<usize>().unwrap();
    let filtro = matches.value_of("filtro").unwrap_or("links");
    let formato = matches.value_of("formato").unwrap_or("json");

    println!("URL: {}", url);
    println!("Profundidade: {}", profundidade);
    println!("Filtro: {}", filtro);
    println!("Formato: {}", formato);
}
