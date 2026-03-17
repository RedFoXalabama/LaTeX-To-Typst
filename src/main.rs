mod latex_parser;
mod utils;
mod latex_semantic;

// IMPOSTO PATH PER IL SALVATAGGIO DEGLI OUTPUT INTERMEDI
static INPUT_PATH: &str = "Assets/InputExample/input.tex";
static OUTPUT_PARSETREE_PATH: &str = "Assets/OutputExample/output_ParseTree.txt";
static OUTPUT_AST_PATH: &str = "Assets/OutputExample/output_AST.txt";

// ------------------------------ MAIN EXECUTION ---------------------------------------------------
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Prendo l'input da un file .tex
    let source = latex_parser::read_latex_file(INPUT_PATH)?;

    // Effettuo il parsing del LaTeX, ottenendo un parse tree (Pest Pairs)
    let parse_tree = latex_parser::parse_latex(&source)?;

    // Salvo il parse tree in un file di testo (in formato debug leggibile)
    utils::save_output_file(OUTPUT_PARSETREE_PATH, &format!("{:#?}", parse_tree))?;

    // Costruiamo un AST tramite strutture ed enumerativi astraendo il Parse Tree
    // La definizione della struttura dell'AST si trova in ast_structure
    let ast = latex_semantic::build_ast(parse_tree).map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::InvalidData, format!("{e:?}"))
    })?; // Effettuiamo la mappatura e conversione dell'errore in modo che il main possa restituirlo.
    // Per ora non faccio che il main restituisca un SemanticError, per tenerlo generalizzato in caso di altri errori.

    utils::save_ast_to_file(OUTPUT_AST_PATH, &ast)?;

    Ok(())
}