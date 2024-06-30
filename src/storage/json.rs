use std::fs::OpenOptions;
use std::io::Write;
use serde_json::json;
use fs2::FileExt;
use std::time::Duration;

pub fn save_as_json(resultados: &[String], output: &str) -> std::io::Result<()> {
    if resultados.is_empty() {
        return Ok(()); // Se a lista está vazia, não há nada para fazer
    }

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(output)?;

    // Tentar adquirir o lock, se não conseguir, aguardar e tentar novamente
    loop {
        match file.try_lock_exclusive() {
            Ok(_) => break,
            Err(_) => std::thread::sleep(Duration::from_millis(100)), // Aguarda 100ms antes de tentar novamente
        }
    }
    
    for resultado in resultados {
        if resultado != "" { 
            let json = json!({ "url": resultado });
            writeln!(file, "{}", json)?;
        }
    }

    // Liberar o lock após a escrita
    file.unlock()?;
    Ok(())
}
