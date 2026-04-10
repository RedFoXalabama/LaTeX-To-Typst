use crate::latex_semantic::{AstItemNode, OptionalArgNode, RequiredArgNode};
use crate::codegen::trans_map::translate_items;

pub fn render_comment_block(
    _name: &str,
    _required_args: Vec<RequiredArgNode>,
    _optional_args: Vec<OptionalArgNode>,
    items: Vec<AstItemNode>,
) -> String {
    let mut out = String::new();
    out.push_str("/*");
    out.push_str(&translate_items(items));
    out.push_str("*/");
    out
}