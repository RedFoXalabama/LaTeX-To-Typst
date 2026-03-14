mod latex_parser;
mod utils;
mod latex_semantic;

static INPUT_PATH: &str = "Assets/InputExample/input.tex";
static OUTPUT_PATH: &str = "Assets/OutputExample/output.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = latex_parser::read_latex_file(INPUT_PATH)?;
    let parse_tree = latex_parser::parse_latex(&source)?;

    println!("{:#?}", parse_tree);
    utils::save_output_file(OUTPUT_PATH, &format!("{:#?}", parse_tree))?;

    Ok(())
}