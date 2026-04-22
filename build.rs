fn main() {
    println!("cargo:rerun-if-changed=src/latex_parser/latex.pest");
}
