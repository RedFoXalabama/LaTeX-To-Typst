use pest_derive::Parser;
use pest::iterators::Pairs;
use pest::Parser;
use std::fs;
use std::path::Path;

// Definisce il parser per LaTeX utilizzando pest. La grammatica è definita in un file .pest separato (latex.pest).
// !! ⚠ SE SI VUOLE AGGIORNARE LA GRAMMATICA E NON AVERE ERRORE NELL'IDE PER CHIAMATE A RULE -> RIAVVIARE L'IDE ⚠ !!
// ️ per ora non ho trovato altra soluzione
#[derive(Parser)]
#[grammar = "latex_parser/latex.pest"] // specifica il file .pest che contiene la grammatica
pub struct LatexParser;

// Legge un file LaTeX e ne restituisce il contenuto in formato STRING.
pub fn read_latex_file<P: AsRef<Path>>(path: P) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

// Converte il contenuto LaTeX, in input String, nel parse tree di pest composto da Pairs<Rule>.
pub fn parse_latex<'a>(input: &'a str) -> Result<Pairs<'a, Rule>, pest::error::Error<Rule>> {
    // <'a> definisce un parametro di lifetime generico.
    // In parse_latex, serve a dire che il Pairs<'a, Rule> restituito contiene riferimenti validi finché è valido anche input: &'a str.
    // Quindi lega la durata dell`output` alla durata dell`input`, evitando riferimenti pendenti.
    // In breve: <'a> = “l`output` prende in prestito i dati dall`input` con la stessa durata”.
    LatexParser::parse(Rule::file, input)
}