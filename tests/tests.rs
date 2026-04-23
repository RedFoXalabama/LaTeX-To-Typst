use LaTeX_To_Typst::{codegen, latex_parser, latex_semantic};

fn run_transpilation(input_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let source = latex_parser::read_latex_file(input_path)?;
    latex_parser::scan_latex(&source)?;
    let parse_tree = latex_parser::parse_latex(&source)?;
    let ast = latex_semantic::build_ast(parse_tree)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, format!("{e:?}")))?;
    codegen::validate_ast(&ast)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, format!("{e:?}")))?;
    let _typst_output = codegen::ast_to_typst(&ast);
    Ok(())
}

// ---------------------------------------------------------
// SUCCESS CASES (tests/cases/oks)
// ---------------------------------------------------------

#[test]
fn test_ok_code() {
    let case = "tests/cases/oks/code.tex";
    let result = run_transpilation(case);
    assert!(result.is_ok(), "Failed on test case {}: {:?}", case, result.err());
}

#[test]
fn test_ok_comments() {
    let case = "tests/cases/oks/comments.tex";
    let result = run_transpilation(case);
    assert!(result.is_ok(), "Failed on test case {}: {:?}", case, result.err());
}

#[test]
fn test_ok_figure() {
    let case = "tests/cases/oks/figure.tex";
    let result = run_transpilation(case);
    assert!(result.is_ok(), "Failed on test case {}: {:?}", case, result.err());
}

#[test]
fn test_ok_hyperlinks_test() {
    let case = "tests/cases/oks/hyperlinks_test.tex";
    let result = run_transpilation(case);
    assert!(result.is_ok(), "Failed on test case {}: {:?}", case, result.err());
}

#[test]
fn test_ok_package_test() {
    let case = "tests/cases/oks/package_test.tex";
    let result = run_transpilation(case);
    assert!(result.is_ok(), "Failed on test case {}: {:?}", case, result.err());
}

#[test]
fn test_ok_sections_chapter() {
    let case = "tests/cases/oks/sections_chapter.tex";
    let result = run_transpilation(case);
    assert!(result.is_ok(), "Failed on test case {}: {:?}", case, result.err());
}

#[test]
fn test_ok_space_breaks() {
    let case = "tests/cases/oks/space_breaks.tex";
    let result = run_transpilation(case);
    assert!(result.is_ok(), "Failed on test case {}: {:?}", case, result.err());
}

#[test]
fn test_ok_tables() {
    let case = "tests/cases/oks/tables.tex";
    let result = run_transpilation(case);
    assert!(result.is_ok(), "Failed on test case {}: {:?}", case, result.err());
}

#[test]
fn test_ok_text_alignment() {
    let case = "tests/cases/oks/text_alignment.tex";
    let result = run_transpilation(case);
    assert!(result.is_ok(), "Failed on test case {}: {:?}", case, result.err());
}

#[test]
fn test_ok_text_formatting() {
    let case = "tests/cases/oks/text_formatting.tex";
    let result = run_transpilation(case);
    assert!(result.is_ok(), "Failed on test case {}: {:?}", case, result.err());
}

#[test]
fn test_ok_text_listing() {
    let case = "tests/cases/oks/text_listing.tex";
    let result = run_transpilation(case);
    assert!(result.is_ok(), "Failed on test case {}: {:?}", case, result.err());
}

// ---------------------------------------------------------
// ERROR CASES
// ---------------------------------------------------------

#[test]
fn test_error_inexistent_command() {
    let case = "tests/cases/errors/inexistent_command.tex";
    let result = run_transpilation(case);
    assert!(
        result.is_err(),
        "Expected an error for test case {}, but it succeeded",
        case
    );
}

#[test]
fn test_error_unclosed_bracket() {
    let case = "tests/cases/errors/unclosed_bracket.tex";
    let result = run_transpilation(case);
    assert!(
        result.is_err(),
        "Expected an error for test case {}, but it succeeded",
        case
    );
}

#[test]
fn test_error_unclosed_optional() {
    let case = "tests/cases/errors/unclosed_optional.tex";
    let result = run_transpilation(case);
    assert!(
        result.is_err(),
        "Expected an error for test case {}, but it succeeded",
        case
    );
}

#[test]
fn test_error_unknown_env() {
    let case = "tests/cases/errors/unknown_env.tex";
    let result = run_transpilation(case);
    assert!(
        result.is_err(),
        "Expected an error for test case {}, but it succeeded",
        case
    );
}

#[test]
fn test_error_unmatched_end() {
    let case = "tests/cases/errors/unmatched_end.tex";
    let result = run_transpilation(case);
    assert!(
        result.is_err(),
        "Expected an error for test case {}, but it succeeded",
        case
    );
}