use pest_derive::Parser;
use pest::iterators::Pairs;
use pest::Parser;
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[grammar = "latex_parser/latex.pest"] // specifica il file .pest che contiene la grammatica
pub struct LatexParser;

/// Legge un file LaTeX e ne restituisce il contenuto.
pub fn read_latex_file<P: AsRef<Path>>(path: P) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

/// Converte il contenuto LaTeX nel parse tree di pest.
pub fn parse_latex<'a>(input: &'a str) -> Result<Pairs<'a, Rule>, pest::error::Error<Rule>> {
    LatexParser::parse(Rule::file, input)
}