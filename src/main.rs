use latex_to_typst::{codegen, latex_parser, latex_semantic, utils};
use std::fs;

// ----------------------------------- SAVE PATH FOR DOC -------------------------------------------
static INPUT_DIR: &str = "Assets/Input/";
static PARSETREE_PATH: &str = "Assets/Output/Documentation/output_ParseTree.txt";
static AST_PATH: &str = "Assets/Output/Documentation/output_AST.txt";
static CODEGEN_DIR: &str = "Assets/Output/Documentation/";

// ------------------------------ MAIN EXECUTION ---------------------------------------------------
fn main() {
    // Di default, mostro i log da livello warn in poi. Se interessati a log verbosi da livello info, impostare env RUST_LOG=info
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();

    for file in fs::read_dir(INPUT_DIR).expect("Failed to read input directory") {
        let entry = file.expect("Failed to read directory entry");
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("tex") {
            println!("Translating file: {}", path.display());
            log::info!("Translating file: {}", path.display());

            let input_path = path.to_str().unwrap();
            let codegen_path: &str = &format!(
                "{}{}.typ",
                CODEGEN_DIR,
                path.file_stem().unwrap().to_str().unwrap()
            );

            if let Err(err) = translate_file(input_path, PARSETREE_PATH, AST_PATH, codegen_path) {
                eprintln!("{}", err);
                std::process::exit(1);
            } else {
                log::info!("File translated successfully: {}", codegen_path);
                println!("File translated successfully: {}", codegen_path);
            }
        }
    }
}

fn translate_file(
    input_path: &str,
    output_parsetree_path: &str,
    output_ast_path: &str,
    output_codegen_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // -------------------------------- PARTSING----------------------------------------------------
    // Prendo l'input da un file .tex
    let source = latex_parser::read_latex_file(input_path)?;

    log::info!("1. Latex Input ==> Starting PEST lexical analysis and parse tree construction...");
    // Effettuo il parsing del LaTeX, ottenendo un parse tree (Pest Pairs)
    let parse_tree = latex_parser::parse_latex(&source)?;
    // Salvo il parse tree in un file di testo (in formato debug leggibile)
    utils::save_output_file(output_parsetree_path, &format!("{:#?}", parse_tree))?;

    // ------------------------------- SEMANTIC ANALYSIS -------------------------------------------
    log::info!("3. ParseTree ==> Starting AST construction...");
    // Costruiamo un AST tramite strutture ed enumerativi astraendo il Parse Tree
    // La definizione della struttura dell'AST si trova in ast_structure
    let ast = latex_semantic::build_ast(parse_tree)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, format!("{e:?}")))?; // Effettuiamo la mappatura e conversione dell'errore in modo che il main possa restituirlo.
    utils::save_ast_to_file(output_ast_path, &ast)?;

    // -------------------------------- TYPST GENERATION -------------------------------------------
    log::info!("4. AST ==> Starting Traduction in Typst...");
    let typst_output = codegen::ast_to_typst(&ast);
    utils::save_output_file(output_codegen_path, &typst_output)?;

    log::info!("5. Typst input ==> Starting PDF construction...");
    let _child = utils::start_typst_watch(output_codegen_path)?;
    Ok(())
}
