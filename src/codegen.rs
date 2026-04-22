mod block_trans_map;
pub(crate) mod command_trans_map;
pub mod trans_map;

use crate::codegen::block_trans_map::BlockTransMap;
use crate::codegen::command_trans_map::{CommandTransMap};
use crate::codegen::trans_map::TransMap;
use crate::globals::get_in_listing_value;
use crate::latex_semantic::*;
use crate::utils::{drop_command_warn, COMMANDWARNING};

pub fn ast_to_typst(doc: &AstDocument) -> String {
    println!("3. AST ==> Starting Traduction in Typst...");
    doc.items.iter().map(render_item).collect()
}

pub fn translate_items(items: Vec<AstItemNode>) -> String {
    items.iter().map(render_item).collect()
}

pub(crate) fn render_item(item: &AstItemNode) -> String {
    match item {
        AstItemNode::Block(block_node) => render_block(block_node),
        AstItemNode::Text(text_node) => render_text(text_node),
        AstItemNode::RawText(text_node) => render_raw_text(text_node),
        AstItemNode::Newlines(newlines_node) => render_newlines(newlines_node),
        AstItemNode::Command(command_node) => render_command(command_node),
        AstItemNode::Linebreak(linebreak_node) => render_linebreak(linebreak_node),
        AstItemNode::Comment(comment_node) => render_comment(comment_node),
    }
}

pub(crate) fn render_block(block_node: &BlockNode) -> String {
    if let Some(rendered) = BlockTransMap::translate(block_node) {
        rendered
    } else {
        // Fallback: render the items inside the block normally
        block_node.items.iter().map(render_item).collect()
    }
}

pub(crate) fn render_text(text_node: &TextNode) -> String {
    text_node
        .value
        .chars()
        .filter(|c| *c != '{' && *c != '}')
        .collect()
}

pub(crate) fn render_newlines(newlines_node: &NewlinesNode) -> String {
    let mut count = newlines_node.count;
    if get_in_listing_value() {
        count = count - 1;
    }
    "\n".repeat(count)
}

pub(crate) fn render_linebreak(_linebreak_node: &LinebreakNode) -> String {
    "\\".to_string()
}

pub(crate) fn render_comment(comment_node: &CommentNode) -> String {
    let value = comment_node //eliminiamo il % del commento cosi da conservare solo il testo
        .value
        .strip_prefix('%')
        .unwrap_or(&comment_node.value);
    format!("//{}", value)
}

pub(crate) fn render_command(command_node: &CommandNode) -> String {
    if let Some(rendered) = CommandTransMap::translate(command_node) {
        rendered
    } else {
        let out = String::new();
        drop_command_warn(COMMANDWARNING::WrongCommandOrNotImplemented(command_node.name.clone()),
                          Option::from(out),
                          Option::from(&*command_node.name),
                          Option::from(command_node.required_args.clone()))
    }
}

pub(crate) fn render_raw_text(raw_text_node: &TextNode) -> String {
    raw_text_node.value.clone()
}
