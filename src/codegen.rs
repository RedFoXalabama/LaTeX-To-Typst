pub mod trans_map;
mod command_trans_map;
mod block_trans_map;

use crate::globals::get_in_listing_value;
use crate::latex_semantic::*;
use crate::codegen::trans_map::TransMap;
use crate::codegen::block_trans_map::BlockTransMap;
use crate::codegen::command_trans_map::CommandTransMap;

pub fn ast_to_typst(doc: &AstDocument) -> String {
    doc.items.iter().map(render_item).collect()
}

fn render_item(item: &AstItemNode) -> String {
    match item {
        AstItemNode::Block(block_node) => render_block(block_node),
        AstItemNode::Text(text_node) => render_text(text_node),
        AstItemNode::Newlines(newlines_node) => render_newlines(newlines_node),
        AstItemNode::Command(command_node) => render_command(command_node),
        AstItemNode::Linebreak(linebreak_node) => render_linebreak(linebreak_node), // per ora lascio così, ma potrei renderizzare in modo diverso
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
        "NOT IMPLEMENTED COMMAND RENDER-ERROR".to_string()
    }
}

fn is_begin_tabular(command_node: &CommandNode) -> bool {
    command_node.name == "begin"
        && command_node
            .required_args
            .first()
            .map(|arg| command_trans_map::render_args_item(&arg.items) == "tabular")
            .unwrap_or(false)
}

fn is_end_tabular(command_node: &CommandNode) -> bool {
    command_node.name == "end"
        && command_node
            .required_args
            .first()
            .map(|arg| command_trans_map::render_args_item(&arg.items) == "tabular")
            .unwrap_or(false)
}

fn collect_tabular_body(items: &[AstItemNode], mut index: usize) -> (Vec<AstItemNode>, usize) {
    let mut depth = 0usize;
    let mut body = Vec::new();

    while index < items.len() {
        match &items[index] {
            AstItemNode::Command(command_node) if is_begin_tabular(command_node) => {
                depth += 1;
                body.push(items[index].clone());
            }
            AstItemNode::Command(command_node) if is_end_tabular(command_node) => {
                if depth == 0 {
                    return (body, index + 1);
                }
                depth -= 1;
                body.push(items[index].clone());
            }
            _ => body.push(items[index].clone()),
        }

        index += 1;
    }

    (body, index)
}
