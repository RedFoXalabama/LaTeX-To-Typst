use pest::Parser;
use pest_derive::Parser;
use pest::iterators::Pairs;
use std::fs;
use std::io::Error;
use std::path::Path;
use pest::error::{ErrorVariant, LineColLocation};
use crate::utils::PARSEERROR;

// Definisce il parser per LaTeX utilizzando pest. La grammatica è definita in un file .pest separato (latex.pest).
// !! ⚠ SE SI VUOLE AGGIORNARE LA GRAMMATICA E NON AVERE ERRORE NELL'IDE PER CHIAMATE A RULE -> Cargo clean -> Cargo check -> RIAVVIARE L'IDE ⚠ !!
// ️ per ora non ho trovato altra soluzione
#[derive(Parser)]
#[grammar = "latex_parser/latex.pest"] // specifica il file .pest che contiene la grammatica
pub struct LatexParser;

pub mod math {
    use pest_derive::Parser;
    #[derive(Parser)]
    #[grammar = "latex_parser/latex_math.pest"]
    pub struct LatexMathParser;
}

// Legge un file LaTeX e ne restituisce il contenuto in formato STRING.
pub fn read_latex_file<P: AsRef<Path>>(path: P) -> Result<String, Error> {
    fs::read_to_string(path)
}

// Converte il contenuto LaTeX, in input String, nel parse tree di pest composto da Pairs<Rule>.
pub fn parse_latex<'a>(input: &'a str) -> Result<Pairs<'a, Rule>, Error> {
    // <'a> definisce un parametro di lifetime generico.
    // In parse_latex, serve a dire che il Pairs<'a, Rule> restituito contiene riferimenti validi finché è valido anche input: &'a str.
    // Quindi lega la durata dell`output` alla durata dell`input`, evitando riferimenti pendenti.
    // In breve: <'a> = “l`output` prende in prestito i dati dall`input` con la stessa durata”.
    
    LatexParser::parse(Rule::file, input).map_err(|e| {
        // In caso di errore, restituisce un messaggio di errore più leggibile.
        let error_msg = format_pest_error(&e, input);
        Error::new(std::io::ErrorKind::InvalidData, error_msg)
    })
}

// Formatta il messaggio di errore cercando di classificarlo in base alla riga e al contesto del codice LaTeX che ha causato l`errore,
// per fornire un feedback più specifico all`utente.
fn format_pest_error(err: &pest::error::Error<Rule>, input: &str) -> String {
    let (line, col) = match err.line_col {
        LineColLocation::Pos((line, col)) => (line, col),
        LineColLocation::Span((line, col), _) => (line, col),
    };

    let offending_line = input.lines().nth(line.saturating_sub(1)).unwrap_or("");
    let caret_padding = " ".repeat(col.saturating_sub(1));
    let separator = "-".repeat(25);

    let (parse_error, error_msg) = match &err.variant {
        ErrorVariant::ParsingError { positives, negatives } => {
            if !positives.is_empty() {
                let expected = positives
                    .iter()
                    .map(|r| format!("{:?}", r))
                    .collect::<Vec<_>>()
                    .join(", ");
                classify_offending_line(offending_line, expected)
            } else if !negatives.is_empty() {
                let unexpected = negatives
                    .iter()
                    .map(|r| format!("{:?}", r))
                    .collect::<Vec<_>>()
                    .join(", ");
                classify_offending_line(offending_line, unexpected)
            } else {
                (PARSEERROR::GrammaticError, "Errore di parsing".to_string())
            }
        }
        _ => (PARSEERROR::GrammaticError, "Errore di parsing".to_string()),
    };

    format!(
        "{} PARSE ERROR: {:?} {}\nErrore di parsing LaTeX alla riga {}, colonna {}\n{}\n{}^\n{}\n{}",
        separator,
        parse_error,
        separator,
        line,
        col,
        offending_line,
        caret_padding,
        error_msg,
        separator
    )
}

// Implementati al momento gli errori possibili individuati
fn classify_offending_line(line: &str, error_class: String) -> (PARSEERROR, String) {
    let trimmed = line.trim_start_matches('\u{feff}').trim();

    let error = match () {
        _ if trimmed.starts_with("\\begin") => match error_class.as_str() {
            "name, raw_name" => PARSEERROR::MissingEnvironmentName,
            "item" => PARSEERROR::UnmatchedEnvironmentStart,
            "raw_name" => PARSEERROR::InvalidEnvironmentName,
            _ => PARSEERROR::GrammaticError,
        },

        _ if trimmed.starts_with("\\end") => match error_class.as_str() {
            "name, raw_name" => PARSEERROR::MissingEnvironmentName,
            "EOI, item" | "item" => PARSEERROR::UnmatchedEnvironmentEnd,
            _ => PARSEERROR::GrammaticError,
        }

        _ if trimmed.starts_with("\\") => match error_class.as_str() {
            "name" => PARSEERROR::MissingCommandName,
            _ => PARSEERROR::GrammaticError,
        }

        _ => PARSEERROR::GrammaticError,
    };

    (error.clone(), error.message().to_string())
}