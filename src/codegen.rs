mod block_trans_map;
pub(crate) mod command_trans_map;
pub mod trans_map;

use crate::codegen::block_trans_map::BlockTransMap;
use crate::codegen::command_trans_map::CommandTransMap;
use crate::codegen::trans_map::TransMap;
use crate::globals::get_in_listing_value;
use crate::latex_semantic::*;
use crate::utils::{COMMANDWARNING, drop_command_warn};

pub fn ast_to_typst(doc: &AstDocument) -> String {
    doc.items.iter().map(render_item).collect()
}

pub fn validate_ast(doc: &AstDocument) -> Result<(), SemanticError> {
    for item in &doc.items {
        validate_item(item)?;
    }
    Ok(())
}

fn validate_item(item: &AstItemNode) -> Result<(), SemanticError> {
    match item {
        AstItemNode::Block(block_node) => {
            if !BlockTransMap::is_supported(&block_node.name) {
                return Err(SemanticError::UnsupportedCommand(block_node.name.clone()));
            }

            for req in &block_node.required_args {
                validate_req_arg(req)?;
            }
            for opt in &block_node.optional_args {
                validate_opt_arg(opt)?;
            }
            for child in &block_node.items {
                validate_item(child)?;
            }
        }
        AstItemNode::Command(command_node) => {
            if !CommandTransMap::is_supported(&command_node.name) {
                return Err(SemanticError::UnsupportedCommand(command_node.name.clone()));
            }

            // Standard missing arguments validation based on standard LaTeX commands
            let reqs_len = command_node.required_args.len();
            match command_node.name.as_str() {
                "textbf" | "textit" | "underline" | "part" | "chapter" | "section"
                | "subsection" | "subsubsection" | "paragraph" | "subparagraph" | "title"
                | "author" | "date" | "documentclass" | "href" => {
                    if reqs_len < 1 {
                        return Err(SemanticError::MissingArgsForCommand(
                            command_node.name.clone(),
                            1,
                            reqs_len,
                        ));
                    }
                }
                "textcolor" => {
                    if reqs_len < 2 {
                        return Err(SemanticError::MissingArgsForCommand(
                            command_node.name.clone(),
                            2,
                            reqs_len,
                        ));
                    }
                }
                _ => {}
            }

            for req in &command_node.required_args {
                validate_req_arg(req)?;
            }
            for opt in &command_node.optional_args {
                validate_opt_arg(opt)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn validate_req_arg(req_arg: &RequiredArgNode) -> Result<(), SemanticError> {
    for child in &req_arg.items {
        match child {
            ArgItemNode::Command(cmd) => validate_item(&AstItemNode::Command(cmd.clone()))?,
            ArgItemNode::Group(grp) => validate_req_arg(grp)?,
            _ => {}
        }
    }
    Ok(())
}

fn validate_opt_arg(opt_arg: &OptionalArgNode) -> Result<(), SemanticError> {
    for entry in &opt_arg.entries {
        if let OptionalEntryNode::Items(items) = entry {
            for opt_item in items {
                match opt_item {
                    OptItemNode::Command(cmd) => validate_item(&AstItemNode::Command(cmd.clone()))?,
                    OptItemNode::Group(grp) => validate_req_arg(grp)?,
                    _ => {}
                }
            }
        }
    }
    Ok(())
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
        AstItemNode::Whitespace(whitespace_node) => render_whitespace(whitespace_node),
        AstItemNode::Comment(comment_node) => render_comment(comment_node),
    }
}

pub(crate) fn render_whitespace(whitespace_node: &WhitespaceNode) -> String {
    whitespace_node.value.clone()
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
        drop_command_warn(
            COMMANDWARNING::WrongCommandOrNotImplemented(command_node.name.clone()),
            Option::from(out),
            Option::from(&*command_node.name),
            Option::from(command_node.required_args.clone()),
        )
    }
}

pub(crate) fn render_raw_text(raw_text_node: &TextNode) -> String {
    raw_text_node.value.clone()
}
