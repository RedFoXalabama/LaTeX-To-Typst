mod latex_parser;
mod utils;
mod latex_semantic;


static INPUT_PATH: &str = "Assets/InputExample/input.tex";
static OUTPUT_PARSETREE_PATH: &str = "Assets/OutputExample/output_ParseTree.txt";
static OUTPUT_AST_PATH: &str = "Assets/OutputExample/output_AST.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = latex_parser::read_latex_file(INPUT_PATH)?;
    let parse_tree = latex_parser::parse_latex(&source)?;

    // println!("{:#?}", parse_tree);
    utils::save_output_file(OUTPUT_PARSETREE_PATH, &format!("{:#?}", parse_tree))?;

    let ast = latex_semantic::build_ast(parse_tree).map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::InvalidData, format!("{e:?}"))
    })?; // Effettuiamo la mappatura e conversione dell'errore in modo che il main possa restituirlo.
    // Per ora non faccio che il main restituisca un SemanticError, per tenerlo generalizzato in caso di altri errori.

    utils::save_ast_to_file(OUTPUT_AST_PATH, &ast)?;

    Ok(())
}