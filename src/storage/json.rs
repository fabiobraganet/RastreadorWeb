use std::fs::OpenOptions;
use std::io::Write;
use serde_json::json;

pub fn save_as_json(resultados: &[String], output: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(output)?;
    
    for resultado in resultados {
        let json = json!({ "url": resultado });
        writeln!(file, "{}", json)?;
    }
    Ok(())
}
