use std::fs::OpenOptions;
use std::io::Write;

pub fn save_as_csv(resultados: &[String], output: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(output)?;
    
    for resultado in resultados {
        writeln!(file, "{}", resultado)?;
    }
    Ok(())
}
