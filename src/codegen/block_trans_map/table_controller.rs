use crate::codegen::command_trans_map::render_args_item;
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

    let n_cols = count_columns(required_args.first()).max(1);
    out.push_str(&format!("#table(\n\tcolumns: {},\n", n_cols));

    for entry in build_table_entries(&items) {
        match entry {
            TableEntry::Row(cells) => {
                for cell in cells {
                    out.push_str(&format!("\t[{}],\n", cell));
                }
            }
            TableEntry::HLine => {} // Tabelle con righe mancanti non sono gestite per ora
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

    for item in items {
        match item {
            AstItemNode::Text(text) => {
                // Here we might need to handle & directly, but text is already parsed.
                // In AstItemNode::Text, `&` is just part of the text!
                let txt_val: Vec<String> = text
                    .value
                    .clone()
                    .split('&')
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .map(String::from)
                    .collect();

                for txt in txt_val {
                    entries.push(TableEntry::Row(vec![txt]));
                }
            }
            AstItemNode::Command(command) if command.name == "hline" => {
                entries.push(TableEntry::HLine);
            }
            AstItemNode::Linebreak(_) | AstItemNode::Newlines(_) | AstItemNode::Comment(_) => {}
            AstItemNode::Command(command) => {
                let rendered = crate::codegen::render_command(command);
                if !rendered.is_empty() {
                    entries.push(TableEntry::Row(vec![rendered]));
                }
            }
            AstItemNode::Block(b) => {
                // If there's a nested block
                let rendered = crate::codegen::render_block(b);
                if !rendered.is_empty() {
                    entries.push(TableEntry::Row(vec![rendered]));
                }
            }
        }
    }

    entries
}
