mod trans_map;
use crate::globals::get_in_listing_value;
use crate::latex_semantic::*;

pub fn ast_to_typst(doc: &AstDocument) -> String {
    let mut out = String::new();
    let mut index = 0;
    while index < doc.items.len() {
        let item = &doc.items[index];
        if let AstItemNode::Command(cmd) = item {
            if is_begin_tabular(cmd) {
                let config_arg = cmd.required_args.get(1); // tabular usually has {cols}
                let (body, next_index) = collect_tabular_body(&doc.items, index + 1);
                out.push_str(&trans_map::tables::render_table(config_arg, &body));
                index = next_index;
                continue;
            }
        }
        out.push_str(&render_item(item));
        index += 1;
    }
    out
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
    if let Some(rendered) = trans_map::translate_block(block_node) {
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
    if let Some(rendered) = trans_map::translate_command(command_node) {
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
            .map(|arg| trans_map::render_args_item(&arg.items) == "tabular")
            .unwrap_or(false)
}

fn is_end_tabular(command_node: &CommandNode) -> bool {
    command_node.name == "end"
        && command_node
            .required_args
            .first()
            .map(|arg| trans_map::render_args_item(&arg.items) == "tabular")
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
