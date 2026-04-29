use std::fs;
use std::path::Path;
use crate::latex_semantic::{AstDocument, RequiredArgNode};
use std::io;
use std::process::{Child, Command, Stdio};
use log::warn;

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

// Funzione che renderizza il file .typ in .pdf
// Bisogna aver installato il comando typst watch sulla propria macchina
pub fn start_typst_watch(typ_path: &str) -> io::Result<Child> {
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

// -------------------------------- GESTIONE ERRORI ------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum COMMANDWARNING {
    NotImplemented(String),
    WrongCommandOrNotImplemented(String),
    EmptyBracket(String),
    EnvironmentBlockNotImplemented(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PARSEERROR {
    GrammaticError,
    MissingEnvironmentName,
    InvalidEnvironmentName,
    MissingCommandName,
    UnmatchedEnvironmentEnd,
    UnmatchedEnvironmentStart,
}

impl PARSEERROR {
    pub fn message(&self) -> &'static str {
        match self {
            PARSEERROR::GrammaticError => "Errore di grammatica generico",
            PARSEERROR::MissingEnvironmentName => "Assenza nome d'ambiente",
            PARSEERROR::InvalidEnvironmentName => "Nome d'ambiente non valido",
            PARSEERROR::MissingCommandName => "Assenza nome comando",
            PARSEERROR::UnmatchedEnvironmentEnd => "Ambiente aperto erroneamente o Apertura mancante",
            PARSEERROR::UnmatchedEnvironmentStart => "Ambiente chiuso erroneamente o Chiusura mancante",
        }
    }
}

// Funzione per generare i warning durante la fase di rendering, in caso di comandi o ambienti non implementati o errori di sintassi
// Genera sia un warn sulla console, sia una stringa da inserire nel file di output in modo da poter sapere dove e cosa ha generato il warn
pub fn drop_command_warn(
    warn: COMMANDWARNING,
    out: Option<String>,
    name: Option<&str>,
    reqs: Option<Vec<RequiredArgNode>>,
) -> String {
    let mut out = out.unwrap_or_default();

    match &warn {
        COMMANDWARNING::NotImplemented(why)
        | COMMANDWARNING::WrongCommandOrNotImplemented(why) => {
            let cmd_name = name.unwrap_or("<unknown>");
            let rendered_args = reqs
                .unwrap_or_default()
                .iter()
                .map(|r| crate::codegen::command_trans_map::render_args_item(&r.items))
                .collect::<Vec<_>>()
                .join("}{");

            let error_msg = format!(
                "WARN: {:?}: {} \\{}{{{}}}",
                warn, why, cmd_name, rendered_args
            );
            warn!("==> {}", error_msg);
            out.push_str(&format!("/*{}*/", error_msg));
        }
        
        COMMANDWARNING::EmptyBracket(why) => {
            warn!(
                "EmptyBracket(\\{}): expected at least 1 argument item, found EMPTY BRACKET",
                why
            );
        }

        COMMANDWARNING::EnvironmentBlockNotImplemented(why) => {
            let error_msg = format!(
                "WARN: {:?}: Environment block '{}' not implemented or wrong",
                warn, why
            );
            warn!("==> {}", error_msg);
            out.push_str(&format!("/*{}*/", error_msg));
        }
    }

    out
}