use crate::latex_semantic::{AstItemNode, OptionalArgNode, RequiredArgNode};
use crate::codegen::trans_map::translate_items;
use crate::globals::{ListType, add_in_listing_priority, pop_in_listing_priority};

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

pub fn render_itemize_block(
    _name: &str,
    _required_args: Vec<RequiredArgNode>,
    _optional_args: Vec<OptionalArgNode>,
    items: Vec<AstItemNode>,
) -> String {
    add_in_listing_priority(ListType::Itemize);
    let out = translate_items(items);
    pop_in_listing_priority();
    out
}

pub fn render_enumerate_block(
    _name: &str,
    _required_args: Vec<RequiredArgNode>,
    _optional_args: Vec<OptionalArgNode>,
    items: Vec<AstItemNode>,
) -> String {
    add_in_listing_priority(ListType::Enumerate);
    let out = translate_items(items);
    pop_in_listing_priority();
    out
}

pub fn render_description_block(
    _name: &str,
    _required_args: Vec<RequiredArgNode>,
    _optional_args: Vec<OptionalArgNode>,
    items: Vec<AstItemNode>,
) -> String {
    add_in_listing_priority(ListType::Description);
    let out = translate_items(items);
    pop_in_listing_priority();
    out
}

pub fn render_document_block(
    _name: &str,
    _required_args: Vec<RequiredArgNode>,
    _optional_args: Vec<OptionalArgNode>,
    items: Vec<AstItemNode>,
) -> String {
    translate_items(items)
}
