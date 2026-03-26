mod trans_map;
use crate::globals::{get_in_listing_value};
use crate::latex_semantic::*;

pub fn ast_to_typst(doc: &AstDocument) -> String {
    doc.items.iter().map(render_item).collect()
}

fn render_item(item: &AstItemNode) -> String {
    match item {
        AstItemNode::Text(text_node) => render_text(text_node),
        AstItemNode::Newlines(newlines_node) => render_newlines(newlines_node),
        AstItemNode::Command(command_node) => render_command(command_node),
        AstItemNode::Linebreak(linebreak_node) => render_linebreak(linebreak_node), // per ora lascio così, ma potrei renderizzare in modo diverso
        AstItemNode::Comment(comment_node) => render_comment(comment_node),
    }
}

fn render_text(text_node: &TextNode) -> String {
    text_node
        .value
        .chars()
        .filter(|c| *c != '{' && *c != '}')
        .collect()
}

fn render_newlines(newlines_node: &NewlinesNode) -> String {
    let mut count = newlines_node.count;
    if get_in_listing_value() {
        count = count - 1;
    }
    "\n".repeat(count)
}

fn render_linebreak(_linebreak_node: &LinebreakNode) -> String {
    "\\".to_string()
}

fn render_comment(comment_node: &CommentNode) -> String {
    let value = comment_node //eliminiamo il % del commento cosi da conservare solo il testo
        .value
        .strip_prefix('%')
        .unwrap_or(&comment_node.value);
    format!("//{}", value)
}

fn render_command(command_node: &CommandNode) -> String {
    if let Some(rendered) = trans_map::translate_command(command_node) {
        rendered
    } else {
        "NOT IMPLEMENTED COMMAND RENDER-ERROR".to_string()
    }
}
