use latex_to_typst::{codegen, latex_parser, latex_semantic}; // Accedo al file lib.rs

fn run_transpilation(input_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let source = latex_parser::read_latex_file(input_path)?;
    let parse_tree = latex_parser::parse_latex(&source)?;
    let ast = latex_semantic::build_ast(parse_tree)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, format!("{e:?}")))?;
    let typst_output = codegen::ast_to_typst(&ast);
    Ok(typst_output)
}

macro_rules! test_ok {
    ($name:ident, $file:expr) => {
        #[test]
        fn $name() {
            let case = format!("tests/cases/oks/{}.tex", $file);
            let assertion = format!("tests/assertions/oks/{}/{}_output.typ", $file, $file);
            let result_path = format!("tests/results/oks/{}/{}_output.typ", $file, $file);

            let expected = std::fs::read_to_string(&assertion).expect("Missing assertion file");
            let result = run_transpilation(&case);

            // Mi aspetto che la traduzione termini con successo
            assert!(result.is_ok(), "Failed on test case {}: {:?}", case, result.err());
            let actual = result.unwrap();

            // Salvo il risultato per facilitare debugging
            if let Some(parent) = std::path::Path::new(&result_path).parent() {
                std::fs::create_dir_all(parent).expect("Failed to create results directory");
            }
            std::fs::write(&result_path, &actual).expect("Failed to write result file");

            // Confronto il documento appena generato con quello atteso
            assert_eq!(actual, expected, "Mismatch in {}", case);
        }
    };
}

macro_rules! test_error {
    ($name:ident, $file:expr) => {
        #[test]
        fn $name() {
            let case = format!("tests/cases/errors/{}.tex", $file);
            let assertion = format!("tests/assertions/errors/{}/{}_output.typ", $file, $file);
            let result_path = format!("tests/results/errors/{}/{}_output.typ", $file, $file);

            let result = run_transpilation(&case);

            if std::path::Path::new(&assertion).exists() {
                // Se esiste un file di asserzione, mi aspetto che la traduzione termini con successo
                let expected = std::fs::read_to_string(&assertion).expect("Failed to read assertion file");
                assert!(result.is_ok(), "Failed on test case {}: {:?}", case, result.err());
                let actual = result.unwrap();

                // Salvo il risultato per facilitare debugging
                if let Some(parent) = std::path::Path::new(&result_path).parent() {
                    std::fs::create_dir_all(parent).expect("Failed to create results directory");
                }
                std::fs::write(&result_path, &actual).expect("Failed to write result file");

                // Confronto il documento appena generato con quello atteso
                assert_eq!(actual, expected, "Mismatch in {}", case);
            } else {
                // Altrimenti, mi aspetto che la traduzione fallisca
                assert!(
                    result.is_err(),
                    "Expected an error for test case {}, but it succeeded",
                    case
                );
            }
        }
    };
}

// ---------------------------------------------------------
// SUCCESS CASES (tests/cases/oks)
// ---------------------------------------------------------

test_ok!(test_ok_code, "code");
test_ok!(test_ok_comments, "comments");
test_ok!(test_ok_figure, "figure");
test_ok!(test_ok_hyperlinks, "hyperlinks");
test_ok!(test_ok_package, "package");
test_ok!(test_ok_sections_chapter, "sections_chapter");
test_ok!(test_ok_space_breaks, "space_breaks");
test_ok!(test_ok_tables, "tables");
test_ok!(test_ok_text_alignment, "text_alignment");
test_ok!(test_ok_text_formatting, "text_formatting");
test_ok!(test_ok_text_listing, "text_listing");

// ---------------------------------------------------------
// ERROR CASES
// ---------------------------------------------------------

test_error!(test_error_unknown_command, "unknown_command");
test_error!(test_error_unclosed_bracket, "unclosed_bracket");
test_error!(test_error_unclosed_optional, "unclosed_optional");
test_error!(test_error_unknown_env, "unknown_env");
test_error!(test_error_unmatched_end, "unmatched_end");
