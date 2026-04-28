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

fn format_pest_error(err: &pest::error::Error<Rule>, input: &str) -> String {
    let (line, col) = match err.line_col {
        LineColLocation::Pos((line, col)) => (line, col),
        LineColLocation::Span((line, col), _) => (line, col),
    };

    let offending_line = input.lines().nth(line.saturating_sub(1)).unwrap_or("");
    let caret_padding = " ".repeat(col.saturating_sub(1));
    let separator = "-".repeat(25);

    let message = match &err.variant {
        ErrorVariant::ParsingError { positives, negatives } => {
            if !positives.is_empty() {
                let expected = positives
                    .iter()
                    .map(|r| format!("{:?}", r))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("Atteso: {}", expected)
            } else if !negatives.is_empty() {
                let unexpected = negatives
                    .iter()
                    .map(|r| format!("{:?}", r))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("Non atteso: {}", unexpected)
            } else {
                "Errore di parsing".to_string()
            }
        }
        _ => "Errore di parsing".to_string(),
    };

    format!(
        "{} PARSE ERROR: {:?} {}\nErrore di parsing LaTeX alla riga {}, colonna {}\n{}\n{}^",
        separator,
        PARSEERROR::GrammaticError,
        separator,
        line,
        col,
        offending_line,
        caret_padding
    ) + &format!("\n{}\n{}", message, separator)
}