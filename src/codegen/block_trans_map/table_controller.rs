use crate::codegen::command_trans_map::render_args_item;
use crate::codegen::render_command;
use crate::latex_semantic::{AstItemNode, OptionalArgNode, RequiredArgNode};

#[derive(Debug)]
enum TableEntry {
    Row(Vec<String>),
    HLine,
}

pub fn render_table(
    _name: &str,
    required_args: Vec<RequiredArgNode>,
    _optional_args: Vec<OptionalArgNode>,
    items: Vec<AstItemNode>,
) -> String {
    let mut out = String::new();
    let columns = count_columns(required_args.first()).max(1);

    out.push_str(&format!("#table(\n\tcolumns: {},\n", columns));

    for entry in build_table_entries(&items) {
        match entry {
            TableEntry::Row(cells) => {
                for cell in cells {
                    out.push_str(&format!("\t[{}],\n", cell));
                }
            }
            TableEntry::HLine => {}
        }
    }

    out.push(')');
    out
}

fn count_columns(arg: Option<&RequiredArgNode>) -> usize {
    arg.map(|req| {
        render_args_item(&req.items)
            .chars()
            .filter(|&col| matches!(col, 'c' | 'l' | 'r'))
            .count()
    })
    .unwrap_or(1)
}

fn build_table_entries(items: &[AstItemNode]) -> Vec<TableEntry> {
    let mut entries = Vec::new();
    let mut current_row: Vec<String> = Vec::new();
    let mut current_cell = String::new();
    let mut row_started = false;

    for item in items {
        match item {
            AstItemNode::Text(text) => {
                // Here we might need to handle & directly, but text is already parsed.
                // In AstItemNode::Text, `&` is just part of the text!
                let txt_val = text.value.clone();
                append_text_to_cell(
                    &txt_val,
                    &mut current_row,
                    &mut current_cell,
                    &mut row_started,
                );
            }
            AstItemNode::Command(command) if command.name == "hline" => {
                finalize_row(
                    &mut entries,
                    &mut current_row,
                    &mut current_cell,
                    &mut row_started,
                );
                entries.push(TableEntry::HLine);
            }
            AstItemNode::Linebreak(_) => {
                finalize_row(
                    &mut entries,
                    &mut current_row,
                    &mut current_cell,
                    &mut row_started,
                );
            }
            AstItemNode::Newlines(_) | AstItemNode::Comment(_) => {}
            AstItemNode::Command(command) => {
                let rendered = render_command(command);
                if !rendered.is_empty() {
                    current_cell.push_str(&rendered);
                    row_started = true;
                }
            }
            AstItemNode::Block(b) => {
                // If there's a nested block
                let rendered = crate::codegen::render_block(b);
                current_cell.push_str(&rendered);
                row_started = true;
            }
        }
    }

    finalize_row(
        &mut entries,
        &mut current_row,
        &mut current_cell,
        &mut row_started,
    );
    entries
}

fn append_text_to_cell(
    text: &str,
    current_row: &mut Vec<String>,
    current_cell: &mut String,
    row_started: &mut bool,
) {
    let mut parts = text.split('&');
    if let Some(first) = parts.next() {
        current_cell.push_str(first);
        if !first.trim().is_empty() {
            *row_started = true;
        }
    }

    for part in parts {
        push_cell(current_row, current_cell);
        *row_started = true;
        current_cell.push_str(part);
    }
}

fn push_cell(current_row: &mut Vec<String>, current_cell: &mut String) {
    current_row.push(current_cell.trim().to_string());
    current_cell.clear();
}

fn finalize_row(
    entries: &mut Vec<TableEntry>,
    current_row: &mut Vec<String>,
    current_cell: &mut String,
    row_started: &mut bool,
) {
    if !*row_started && current_row.is_empty() && current_cell.trim().is_empty() {
        current_cell.clear();
        return;
    }

    push_cell(current_row, current_cell);
    entries.push(TableEntry::Row(std::mem::take(current_row)));
    *row_started = false;
}
