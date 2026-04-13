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

pub fn translate_items(items: Vec<AstItemNode>) -> String {
    items.iter().map(render_item).collect()
}

// pub fn translate_items_raw(items: Vec<AstItemNode>) -> String {
//     items.iter().map(render_item_raw).collect()
// }

pub(crate) fn render_item(item: &AstItemNode) -> String {
    match item {
        AstItemNode::Block(block_node) => render_block(block_node),
        AstItemNode::Text(text_node) => render_text(text_node),
        AstItemNode::Newlines(newlines_node) => render_newlines(newlines_node),
        AstItemNode::Command(command_node) => render_command(command_node),
        AstItemNode::Linebreak(linebreak_node) => render_linebreak(linebreak_node),
        AstItemNode::Comment(comment_node) => render_comment(comment_node),
    }
}

// pub(crate) fn render_item_raw(item: &AstItemNode) -> String {
//     match item {
//         AstItemNode::Block(block_node) => render_block_raw(block_node),
//         AstItemNode::Text(text_node) => text_node.value.clone(),
//         AstItemNode::Newlines(newlines_node) => "\n".repeat(newlines_node.count),
//         AstItemNode::Command(command_node) => render_command_raw(command_node),
//         AstItemNode::Linebreak(linebreak_node) => linebreak_node.value.clone(),
//         AstItemNode::Comment(comment_node) => comment_node.value.clone(),
//     }
// }

pub(crate) fn render_block(block_node: &BlockNode) -> String {
    if let Some(rendered) = BlockTransMap::translate(block_node) {
        rendered
    } else {
        // Fallback: render the items inside the block normally
        block_node.items.iter().map(render_item).collect()
    }
}

// pub(crate) fn render_block_raw(block_node: &BlockNode) -> String {
//     let mut out = String::new();
//     out.push_str(&format!("\\begin{{{}}}", block_node.name));
//     // Reconstruct arguments if any (though usually not in verbatim, but for completeness)
//     for arg in &block_node.required_args {
//         out.push_str(&format!("{{{}}}", render_args_item_raw(&arg.items)));
//     }
//     out.push_str(&translate_items_raw(block_node.items.clone()));
//     out.push_str(&format!("\\end{{{}}}", block_node.name));
//     out
// }

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

pub(crate) fn render_command_raw(command_node: &CommandNode) -> String {
    let mut out = String::new();
    out.push_str(&format!("\\{}", command_node.name));
    for arg in &command_node.required_args {
        out.push_str(&format!("{{{}}}", render_args_item_raw(&arg.items)));
    }
    out
}

fn render_args_item_raw(seq: &Vec<ArgItemNode>) -> String {
    seq.iter()
        .map(|item| match item {
            ArgItemNode::Command(cmd) => render_command_raw(cmd),
            ArgItemNode::Group(group) => format!("{{{}}}", render_args_item_raw(&group.items)),
            ArgItemNode::Newlines(newlines) => "\n".repeat(newlines.count),
            ArgItemNode::Linebreak(linebreak) => linebreak.value.clone(),
            ArgItemNode::Text(text) => text.value.clone(),
        })
        .collect()
}


