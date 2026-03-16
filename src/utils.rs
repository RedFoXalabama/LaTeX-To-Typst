use std::fs;
use std::path::Path;
use crate::latex_semantic::AstDocument;

pub fn save_output_file<P: AsRef<Path>>(path: P, content: &str) -> Result<(), std::io::Error> {
    let path = path.as_ref();

    // Crea la cartella parent se non esiste (es. "OutputExample/")
    if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, content)
}

// Salva l'AST in formato debug leggibile (pretty)
pub fn save_ast_to_file<P: AsRef<Path>>(path: P, ast: &AstDocument) -> Result<(), std::io::Error> {
    let path = path.as_ref();

    if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, format!("{:#?}", ast))
}