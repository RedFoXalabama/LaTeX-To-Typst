use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "latex.pest"]
pub struct LatexParser;

fn main() {
    // Il nostro test case
    let input = "Ciao \\textbf{Mondo}";

    // 1. Parsing
    let file = LatexParser::parse(Rule::file, input)
        .expect("Errore nel parsing") // Se fallisce, il programma si ferma qui
        .next()
        .unwrap();

    // 2. Elaborazione dei risultati
    let mut final_output = String::new();

    for item in file.into_inner() {
        match item.as_rule() {
            Rule::text => {
                final_output.push_str(item.as_str());
            }
            Rule::command => {
                // Entriamo dentro il comando per estrarre nome e argomento
                let mut inner_rules = item.into_inner();
                let cmd_name = inner_rules.next().unwrap().as_str();
                let cmd_arg  = inner_rules.next().unwrap().as_str();

                // Traduzione brutale
                if cmd_name == "textbf" {
                    final_output.push_str(&format!("*{}*", cmd_arg));
                } else {
                    final_output.push_str(&format!("[UNKNOWN CMD: {}]", cmd_name));
                }
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    println!("Originale: {}", input);
    println!("Typst:     {}", final_output);
}