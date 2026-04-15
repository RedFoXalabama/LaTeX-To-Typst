mod codegen;
mod latex_parser;
mod latex_semantic;
mod utils;
mod globals;

// ----------------------------------- SAVE PATH FOR DOC -------------------------------------------
static DOC_PATH: &str = "Assets/Input/Documentation/doc.tex";
//static DOC_PATH: &str = "Assets/Input/input.tex";
static DOC_PARSETREE_PATH: &str = "Assets/Output/Documentation/output_ParseTree.txt";
static DOC_AST_PATH: &str = "Assets/Output/Documentation/output_AST.txt";
static DOC_CODEGEN_PATH: &str = "Assets/Output/Documentation/doc.typ";


// -------------------------------- SAVE PATH FOR ERROR --------------------------------------------
static ERROR_DOC_PATH: &str = "Assets/Input/ErrorCase/error_case.tex";
static ERROR_DOC_PARSETREE_PATH: &str = "Assets/Output/ErrorCase/error_output_ParseTree.txt";
static ERROR_DOC_AST_PATH: &str = "Assets/Output/ErrorCase/error_output_AST.txt";
static ERROR_DOC_CODEGEN_PATH: &str = "Assets/Output/ErrorCase/error_output_codegen.typ";


// FLAG DI CONFIGURAZIONE PER LE RUN
fn env_flag(name: &str) -> bool {
    std::env::var(name)
        .ok()
        .map(|v| matches!(v.trim().to_ascii_lowercase().as_str(), "1" | "true" | "yes" | "on"))
        .unwrap_or(false)
}

// ------------------------------ MAIN EXECUTION ---------------------------------------------------
fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init(); // EVENT LOGGER PER GLI ERRORI

    // ----------------------------------- RENDERING DOC -------------------------------------------
    if env_flag("RUN_DOC_CASE") {
        translate_file(
            DOC_PATH,
            DOC_PARSETREE_PATH,
            DOC_AST_PATH,
            DOC_CODEGEN_PATH,
        )?;
    }

    // ----------------------------------- RENDERING ERROR DOC -------------------------------------
    if env_flag("RUN_ERROR_CASES") {
        translate_file(
            ERROR_DOC_PATH,
            ERROR_DOC_PARSETREE_PATH,
            ERROR_DOC_AST_PATH,
            ERROR_DOC_CODEGEN_PATH,
        )?;
    }


    Ok(())
}

fn translate_file(input_path: &str, output_parsetree_path: &str, output_ast_path: &str, output_codegen_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // -------------------------------- PARTSING----------------------------------------------------
    // Prendo l'input da un file .tex
    let source = latex_parser::read_latex_file(input_path)?;
    // Effettuo il parsing del LaTeX, ottenendo un parse tree (Pest Pairs)
    let parse_tree = latex_parser::parse_latex(&source)?;
    // Salvo il parse tree in un file di testo (in formato debug leggibile)
    utils::save_output_file(output_parsetree_path, &format!("{:#?}", parse_tree))?;

    // ------------------------------- SEMANTIC ANALYSIS -------------------------------------------
    // Costruiamo un AST tramite strutture ed enumerativi astraendo il Parse Tree
    // La definizione della struttura dell'AST si trova in ast_structure
    let ast = latex_semantic::build_ast(parse_tree)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, format!("{e:?}")))?; // Effettuiamo la mappatura e conversione dell'errore in modo che il main possa restituirlo.
    utils::save_ast_to_file(output_ast_path, &ast)?;

    // -------------------------------- TYPST GENERATION -------------------------------------------
    let typst_output = codegen::ast_to_typst(&ast);
    utils::save_output_file(output_codegen_path, &typst_output)?;

    Ok(())
}