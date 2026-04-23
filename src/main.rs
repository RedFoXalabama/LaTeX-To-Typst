use std::fs;
use std::path::{Path, PathBuf};

use LaTeX_To_Typst::{codegen, latex_parser, latex_semantic, utils};

// ----------------------------------- SAVE PATH FOR DOC -------------------------------------------
static DOC_PATH: &str = "Assets/Input/Documentation/doc.tex";
//static DOC_PATH: &str = "Assets/Input/input.tex";
static DOC_PARSETREE_PATH: &str = "Assets/Output/Documentation/output_ParseTree.txt";
static DOC_AST_PATH: &str = "Assets/Output/Documentation/output_AST.txt";
static DOC_CODEGEN_PATH: &str = "Assets/Output/Documentation/doc.typ";


// -------------------------------- SAVE PATH FOR ERROR --------------------------------------------
static ERROR_DOC_PATH: &str = "Assets/Input/ErrorCases/error_case.tex";
static ERROR_DOC_PARSETREE_PATH: &str = "Assets/Output/ErrorCases/error_output_ParseTree.txt";
static ERROR_DOC_AST_PATH: &str = "Assets/Output/ErrorCases/error_output_AST.txt";
static ERROR_DOC_CODEGEN_PATH: &str = "Assets/Output/ErrorCases/error_output_codegen.typ";


// FLAG DI CONFIGURAZIONE PER LE RUN
fn env_flag(name: &str) -> bool {
    std::env::var(name)
        .ok()
        .map(|v| matches!(v.trim().to_ascii_lowercase().as_str(), "1" | "true" | "yes" | "on"))
        .unwrap_or(false)
}

// ------------------------------ MAIN EXECUTION ---------------------------------------------------
fn main() {
    env_logger::init(); // EVENT LOGGER PER GLI ERRORI

    if let Err(err) = run() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    // ----------------------------------- RENDERING DOC -------------------------------------------
    if env_flag("RUN_DOC_CASE") {
        translate_file(
            DOC_PATH,
            DOC_PARSETREE_PATH,
            DOC_AST_PATH,
            DOC_CODEGEN_PATH,
        )?;
    }

    // ----------------------------------- RENDERING TEST CASE DOC ---------------------------------
    if env_flag("RUN_TEST_CASES") {
        translate_all_test_cases(
            "Assets/Input/TestCases",
            "Assets/Output/TestCases",
        )?;
    }

    // ------------------------------- RENDERING ERROR CASE ----------------------------------------
    if env_flag("ERROR_CASES") {
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

    // Scansione preliminare per eventuali errori formali semplici
    latex_parser::scan_latex(&source)?;

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

    // Validazione AST (es. comandi supportati)
    codegen::validate_ast(&ast)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, format!("{e:?}")))?;

    // -------------------------------- TYPST GENERATION -------------------------------------------
    let typst_output = codegen::ast_to_typst(&ast);
    utils::save_output_file(output_codegen_path, &typst_output)?;

    let _child = utils::start_typst_watch(output_codegen_path)?;
    Ok(())
}

// ------------------------------------- TEST CASES TRANSLATION ------------------------------------
fn translate_all_test_cases(
    input_testcases_dir: &str,
    output_testcases_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let input_dir = Path::new(input_testcases_dir);
    let output_dir = Path::new(output_testcases_dir);

    if !input_dir.exists() {
        return Err(format!("Input TestCases directory not found: {}", input_dir.display()).into());
    }

    // Raccoglie tutti i file .tex nella cartella
    let mut tex_files: Vec<PathBuf> = fs::read_dir(input_dir)?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|p| p.is_file())
        .filter(|p| {
            p.extension()
                .and_then(|e| e.to_str())
                .map(|e| e.eq_ignore_ascii_case("tex"))
                .unwrap_or(false)
        })
        .collect();

    // Ordinamento per output deterministico
    tex_files.sort();

    for input_file in tex_files {
        let stem = input_file
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| format!("Invalid tests file name: {}", input_file.display()))?;

        let case_output_dir = output_dir.join(stem);
        let output_parse = case_output_dir.join(format!("{}_output_ParseTree.txt", stem));
        let output_ast = case_output_dir.join(format!("{}_output_AST.txt", stem));
        let output_typ = case_output_dir.join(format!("{}_output.typ", stem));

        translate_file(
            &input_file.to_string_lossy(),
            &output_parse.to_string_lossy(),
            &output_ast.to_string_lossy(),
            &output_typ.to_string_lossy(),
        )?;
    }

    Ok(())
}