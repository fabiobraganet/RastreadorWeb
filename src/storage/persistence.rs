use crate::storage::{csv::save_as_csv, json::save_as_json};

pub fn salvar_resultados(resultados: &mut Vec<String>, formato: &str, output: &str) {
    match formato {
        "json" => save_as_json(resultados, output).expect("Erro ao escrever resultados no arquivo JSON"),
        "csv" => save_as_csv(resultados, output).expect("Erro ao escrever resultados no arquivo CSV"),
        _ => panic!("Formato de saída não suportado!"),
    }
    resultados.clear();
}
