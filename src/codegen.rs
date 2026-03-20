mod trans_map;

use crate::latex_semantic::*;

pub fn ast_to_typst(doc: &AstDocument) -> String {
    doc.items.iter().map(render_item).collect()
}

fn render_item(item: &AstItemNode) -> String {
    match item {
        AstItemNode::Text(text_node) => render_text(text_node),
        AstItemNode::Newlines(newlines_node) => render_newlines(newlines_node),
        AstItemNode::Command(command_node) => render_command(command_node),
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
    "\n".repeat(newlines_node.count)
}

fn render_command(command_node: &CommandNode) -> String {
    if let Some(rendered) = trans_map::translate_command(command_node) {
        rendered
    } else {
        "RENDER-ERROR".to_string()
    }
}
