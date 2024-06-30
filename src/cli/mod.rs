use clap::{App, Arg};

pub fn build_cli() -> App<'static, 'static> {
    App::new("RastreadorWeb")
        .version("1.0")
        .author("Seu Nome <seu.email@example.com>")
        .about("Rastreador de websites que extrai dados")
        .arg(
            Arg::with_name("url")
                .help("A URL para rastrear")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("profundidade")
                .help("A profundidade de links a seguir")
                .short("p")
                .long("profundidade")
                .takes_value(true)
                .default_value("1"),
        )
        .arg(
            Arg::with_name("filtro")
                .help("Filtro para dados específicos (ex: links, imagens)")
                .short("f")
                .long("filtro")
                .takes_value(true)
                .possible_values(&["links", "imagens"]),
        )
        .arg(
            Arg::with_name("formato")
                .help("O formato de saída (json ou csv)")
                .short("o")
                .long("formato")
                .takes_value(true)
                .possible_values(&["json", "csv"])
                .default_value("json"),
        )
        .arg(
            Arg::with_name("output")
                .help("O arquivo de saída")
                .short("O")
                .long("output")
                .takes_value(true)
                .default_value("output"),
        )
}
