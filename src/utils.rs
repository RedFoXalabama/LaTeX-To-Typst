use std::fs;
use std::path::Path;

pub fn save_output_file<P: AsRef<Path>>(path: P, content: &str) -> Result<(), std::io::Error> {
    let path = path.as_ref();

    // Crea la cartella parent se non esiste (es. "OutputExample/")
    if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, content)
}