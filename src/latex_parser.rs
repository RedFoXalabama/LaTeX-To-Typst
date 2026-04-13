use pest::Parser;
use pest_derive::Parser;
use pest::iterators::Pairs;
use std::fs;
use std::path::Path;

// Definisce il parser per LaTeX utilizzando pest. La grammatica è definita in un file .pest separato (latex.pest).
// !! ⚠ SE SI VUOLE AGGIORNARE LA GRAMMATICA E NON AVERE ERRORE NELL'IDE PER CHIAMATE A RULE -> Cargo clean -> Cargo check -> RIAVVIARE L'IDE ⚠ !!
// ️ per ora non ho trovato altra soluzione
#[derive(Parser)]
#[grammar = "latex_parser/latex.pest"] // specifica il file .pest che contiene la grammatica
pub struct LatexParser;

// Legge un file LaTeX e ne restituisce il contenuto in formato STRING.
pub fn read_latex_file<P: AsRef<Path>>(path: P) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

/// Simple scanner to perform preliminary checks on the LaTeX text,
/// such as verifying that all open curly brackets have a closing one.
pub fn scan_latex(input: &str) -> Result<(), std::io::Error> {
    let mut bracket_count: i32 = 0;
    let mut block_stack: Vec<(String, usize, usize)> = Vec::new(); // name, line, col
    let mut line = 1;
    let mut col = 0;

    let mut i = 0;
    let chars: Vec<char> = input.chars().collect();
    
    while i < chars.len() {
        let c = chars[i];
        col += 1;

        if c == '\n' {
            line += 1;
            col = 0;
        }

        if c == '\\' {
            if i + 1 < chars.len() {
                let next_c = chars[i + 1];
                if next_c == '{' || next_c == '}' {
                    i += 2;
                    col += 1;
                    continue;
                }
            }
            
            // Check for \begin{...} and \end{...}
            let remaining: String = chars[i..].iter().collect();
            if remaining.starts_with("\\begin{") {
                if let Some(end_idx) = remaining.find('}') {
                    let name = &remaining[7..end_idx];
                    block_stack.push((name.to_string(), line, col));
                }
            } else if remaining.starts_with("\\end{") {
                if let Some(end_idx) = remaining.find('}') {
                    let name = &remaining[5..end_idx];
                    if let Some((expected_name, expected_line, expected_col)) = block_stack.pop() {
                        if name != expected_name {
                            return Err(std::io::Error::new(
                                std::io::ErrorKind::InvalidData,
                                format!("Scanner Error: Unmatched \\end{{{}}} at line {}, column {}. Expected \\end{{{}}} to match \\begin at line {}, column {}", name, line, col, expected_name, expected_line, expected_col),
                            ));
                        }
                    } else {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            format!("Scanner Error: Found \\end{{{}}} without matching \\begin at line {}, column {}", name, line, col),
                        ));
                    }
                }
            }
        } else if c == '{' {
            bracket_count += 1;
        } else if c == '}' {
            bracket_count -= 1;
            if bracket_count < 0 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Scanner Error: Unmatched closing bracket '}}' at line {}, column {}", line, col),
                ));
            }
        }
        
        i += 1;
    }

    if bracket_count > 0 {
         return Err(std::io::Error::new(
             std::io::ErrorKind::InvalidData,
             format!("Scanner Error: Found {} unclosed opening bracket(s) '{{'", bracket_count),
         ));
    }
    
    if let Some((name, line, col)) = block_stack.pop() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Scanner Error: Unclosed \\begin{{{}}} starting at line {}, column {}", name, line, col),
        ));
    }

    Ok(())
}

// Converte il contenuto LaTeX, in input String, nel parse tree di pest composto da Pairs<Rule>.
pub fn parse_latex<'a>(input: &'a str) -> Result<Pairs<'a, Rule>, pest::error::Error<Rule>> {
    // <'a> definisce un parametro di lifetime generico.
    // In parse_latex, serve a dire che il Pairs<'a, Rule> restituito contiene riferimenti validi finché è valido anche input: &'a str.
    // Quindi lega la durata dell`output` alla durata dell`input`, evitando riferimenti pendenti.
    // In breve: <'a> = “l`output` prende in prestito i dati dall`input` con la stessa durata”.
    LatexParser::parse(Rule::file, input)
}