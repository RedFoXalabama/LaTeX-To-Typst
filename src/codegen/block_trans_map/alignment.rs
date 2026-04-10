use crate::latex_semantic::{AstItemNode, OptionalArgNode, RequiredArgNode};
use crate::codegen::trans_map::translate_items;

pub fn render_center_block(
    _name: &str,
    _required_args: Vec<RequiredArgNode>,
    _optional_args: Vec<OptionalArgNode>,
    items: Vec<AstItemNode>,
) -> String {
    let mut out = String::new();
    out.push_str("#align(center)[");
    out.push_str(&translate_items(items));
    out.push_str("]");
    out
}

pub fn render_flushright_block(
    _name: &str,
    _required_args: Vec<RequiredArgNode>,
    _optional_args: Vec<OptionalArgNode>,
    items: Vec<AstItemNode>,
) -> String {
    let mut out = String::new();
    out.push_str("#align(right)[");
    out.push_str(&translate_items(items));
    out.push_str("]");
    out
}

pub fn render_flushleft_block(
    _name: &str,
    _required_args: Vec<RequiredArgNode>,
    _optional_args: Vec<OptionalArgNode>,
    items: Vec<AstItemNode>,
) -> String {
    let mut out = String::new();
    out.push_str("#align(left)[");
    out.push_str(&translate_items(items));
    out.push_str("]");
    out
}
