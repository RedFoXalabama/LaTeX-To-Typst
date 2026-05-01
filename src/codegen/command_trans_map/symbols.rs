use crate::latex_semantic::{OptionalArgNode, RequiredArgNode};

pub fn render_symbols(
    name: &str,
    _required_args: Vec<RequiredArgNode>,
    _optional_args: Vec<OptionalArgNode>,
) -> String {
    match name {
        "textbackslash" => r"\\".to_string(),
        "textrightarrow" => "#sym.arrow.r".to_string(),
        "textleftarrow" => "#sym.arrow.l".to_string(),
        "_" => r"\_".to_string(),
        "{" => r"\{".to_string(),
        "}" => r"\}".to_string(),
        _ => "".to_string(),
    }
}
