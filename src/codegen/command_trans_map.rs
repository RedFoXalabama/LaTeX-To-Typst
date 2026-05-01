use crate::codegen;
use crate::codegen::trans_map::{CommandTranslationFn, TransMap, render_empty};
use crate::latex_semantic::{
    ArgItemNode, CommandNode, KvPairNode, OptItemNode, OptValueNode, OptionalEntryNode,
    RequiredArgNode,
};
use std::collections::HashMap;
use std::sync::OnceLock;

pub struct CommandTransMap;

mod hyperlinks;
mod package_controller;
mod sections_chapter;
mod space_breaks;
mod text_alignment;
mod text_formatting;
mod text_listing;
mod symbols;

// OnceLock è usato per inizializzare le mappe di traduzione una sola volta, all'avvio del programma.
// Così facendo si evita di evita di ricostruirle ad ogni invocazione di translate_block e translate_command
static TRANS_MAP: OnceLock<HashMap<&'static str, CommandTranslationFn>> = OnceLock::new();

// -------------------------------------------------------------------------------------------------
// --------------------------------------- HASH MAP ------------------------------------------------
// -------------------------------------------------------------------------------------------------
//TODO change strings with enums

impl TransMap<CommandNode> for CommandTransMap {
    fn translate(command: &CommandNode) -> Option<String> {
        let map = get_trans_map();
        map.get(command.name.as_str()).map(|f| {
            f(
                &*command.name,
                command.required_args.clone(),
                command.optional_args.clone(),
            )
        })
    }
}

fn get_trans_map() -> &'static HashMap<&'static str, CommandTranslationFn> {
    TRANS_MAP.get_or_init(|| {
        let mut m = HashMap::new();
        // PACKAGE HANDLER
        m.insert(
            "usepackage",
            package_controller::package_handler as CommandTranslationFn,
        );
        // TEXT FORMATTING
        m.insert(
            "textbf",
            text_formatting::render_formatting as CommandTranslationFn,
        );
        m.insert(
            "textit",
            text_formatting::render_formatting as CommandTranslationFn,
        );
        m.insert(
            "underline",
            text_formatting::render_formatting as CommandTranslationFn,
        );
        m.insert(
            "texttt",
            text_formatting::render_formatting as CommandTranslationFn,
        );
        m.insert(
            "textcolor",
            text_formatting::render_textcolor as CommandTranslationFn,
        );
        m.insert(
            "{",
            text_formatting::render_formatting as CommandTranslationFn,
        );
        m.insert(
            "}",
            text_formatting::render_formatting as CommandTranslationFn,
        );
        m.insert(
            "%",
            text_formatting::render_formatting as CommandTranslationFn,
        );
        m.insert(
            "&",
            text_formatting::render_formatting as CommandTranslationFn,
        );
        m.insert(
            "$",
            text_formatting::render_formatting as CommandTranslationFn,
        );
        m.insert(
            "#",
            text_formatting::render_formatting as CommandTranslationFn,
        );
        m.insert(
            "_",
            text_formatting::render_formatting as CommandTranslationFn,
        );
        // TEXT ALIGNMENT
        m.insert(
            "centering",
            text_alignment::render_document_alignment as CommandTranslationFn,
        );
        m.insert(
            "raggedright",
            text_alignment::render_document_alignment as CommandTranslationFn,
        );
        m.insert(
            "raggedleft",
            text_alignment::render_document_alignment as CommandTranslationFn,
        );
        m.insert(
            "flushright",
            text_alignment::render_document_alignment as CommandTranslationFn,
        );
        m.insert(
            "flushleft",
            text_alignment::render_document_alignment as CommandTranslationFn,
        );
        // SPACE AND BREAKS
        m.insert(
            "newline",
            space_breaks::render_space_breaks as CommandTranslationFn,
        );
        m.insert(
            "break",
            space_breaks::render_space_breaks as CommandTranslationFn,
        );
        m.insert(
            "hfill",
            space_breaks::render_space_breaks as CommandTranslationFn,
        );
        m.insert(
            "vfill",
            space_breaks::render_space_breaks as CommandTranslationFn,
        );
        m.insert(
            "pagebreak",
            space_breaks::render_space_breaks as CommandTranslationFn,
        );
        m.insert(
            "newpage",
            space_breaks::render_space_breaks as CommandTranslationFn,
        );
        m.insert(
            "clearpage",
            space_breaks::render_space_breaks as CommandTranslationFn,
        );
        // LISTING
        m.insert("item", text_listing::render_list as CommandTranslationFn);
        // SECTION CHAPTER TITLE
        m.insert(
            "part",
            sections_chapter::render_section_chapter as CommandTranslationFn,
        );
        m.insert(
            "chapter",
            sections_chapter::render_section_chapter as CommandTranslationFn,
        );
        m.insert(
            "section",
            sections_chapter::render_section_chapter as CommandTranslationFn,
        );
        m.insert(
            "subsection",
            sections_chapter::render_section_chapter as CommandTranslationFn,
        );
        m.insert(
            "subsubsection",
            sections_chapter::render_section_chapter as CommandTranslationFn,
        );
        m.insert(
            "paragraph",
            sections_chapter::render_section_chapter as CommandTranslationFn,
        );
        m.insert(
            "subparagraph",
            sections_chapter::render_section_chapter as CommandTranslationFn,
        );
        m.insert(
            "title",
            sections_chapter::render_info_document as CommandTranslationFn,
        );
        m.insert(
            "maketitle",
            sections_chapter::render_info_document as CommandTranslationFn,
        );
        m.insert(
            "author",
            sections_chapter::render_info_document as CommandTranslationFn,
        );
        m.insert(
            "date",
            sections_chapter::render_info_document as CommandTranslationFn,
        );
        m.insert(
            "today",
            sections_chapter::render_info_document as CommandTranslationFn,
        );
        m.insert(
            "tableofcontents",
            sections_chapter::render_info_document as CommandTranslationFn,
        );
        m.insert(
            "documentclass",
            sections_chapter::render_doc_class as CommandTranslationFn,
        );
        // HYPERLINKS
        m.insert("href", hyperlinks::render_href as CommandTranslationFn);
        // TABLES
        // hlines are handled in the block translation of tabular, so we can ignore the command
        m.insert("hline", render_empty as CommandTranslationFn);
        // SYMBOLS
        m.insert(
            "textbackslash",
            symbols::render_symbols as CommandTranslationFn,
        );
        m.insert(
            "textrightarrow",
            symbols::render_symbols as CommandTranslationFn,
        );
        m.insert(
            "textleftarrow",
            symbols::render_symbols as CommandTranslationFn,
        );
        m
    })
}

// ------------------------------------ ARGUMENT RENDERING------------------------------------------

fn out_of_bounds_reqs_arg(reqs: &[RequiredArgNode], start: usize) -> String {
    let mut extra = String::new();
    for req in reqs.iter().skip(start) {
        extra.push_str(format!(" {}", &render_args_item(&req.items)).as_str());
    }
    extra
}

pub(crate) fn render_args_item(seq: &Vec<ArgItemNode>) -> String {
    seq.into_iter()
        .map(|item| match item {
            ArgItemNode::Command(cmd) => codegen::render_command(&cmd),
            ArgItemNode::Group(group) => render_args_item(&group.items),
            ArgItemNode::Newlines(newlines) => codegen::render_newlines(&newlines),
            ArgItemNode::Linebreak(linebreak) => codegen::render_linebreak(&linebreak),
            ArgItemNode::Text(text) => codegen::render_text(&text),
        })
        .collect()
}

fn render_opt_entry(seq: &Vec<OptionalEntryNode>) -> String {
    seq.into_iter()
        .map(|item| match item {
            OptionalEntryNode::KeyValue(kv_pair) => render_kv_pair(&kv_pair),
            OptionalEntryNode::Items(items) => render_opt_items(&items),
        })
        .collect()
}

fn render_kv_pair(kv_pair: &KvPairNode) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "{}={}",
        kv_pair.key,
        render_kv_value(&kv_pair.value)
    ));
    out
}

fn render_kv_value(value: &OptValueNode) -> String {
    match value {
        OptValueNode::Simple(s) => s.clone(),
        OptValueNode::List(lst) => lst.join(","),
    }
}

fn render_opt_items(items: &Vec<OptItemNode>) -> String {
    items
        .into_iter()
        .map(|item| match item {
            OptItemNode::Command(cmd) => codegen::render_command(&cmd),
            OptItemNode::Group(group) => render_args_item(&group.items),
            OptItemNode::Newlines(newlines) => codegen::render_newlines(&newlines),
            OptItemNode::Text(text) => codegen::render_text(&text),
        })
        .collect()
}
