use std::fs;
use std::path::Path;
use crate::latex_semantic::AstDocument;
use std::io;
use std::process::{Child, Command, Stdio};


// Salva un output in un file di testo, creando la cartella parent se necessario (es. "Output/")
pub fn save_output_file<P: AsRef<Path>>(path: P, content: &str) -> Result<(), std::io::Error> {
    let path = path.as_ref();

    // Crea la cartella parent se non esiste (es. "Output/")
    if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, content)
}

// Salva l'AST in formato debug leggibile
// prende in input una Path ed un AstDocument
pub fn save_ast_to_file<P: AsRef<Path>>(path: P, ast: &AstDocument) -> Result<(), std::io::Error> {
    let path = path.as_ref();

    if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, format!("{:#?}", ast))
}

pub fn start_typst_watch(typ_path: &str) -> io::Result<Child> {
    println!("4. Typst Input ==> Starting PDF construction...");

    if !Path::new(typ_path).exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("File Typst non trovato: {typ_path}"),
        ));
    }

    Command::new("typst")
        .arg("watch")
        .arg(typ_path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
}