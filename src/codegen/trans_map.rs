use crate::codegen;
use crate::latex_semantic::{
    ArgItemNode, AstItemNode, BlockNode, CommandNode, KvPairNode, OptItemNode, OptValueNode,
    OptionalArgNode, OptionalEntryNode, RequiredArgNode,
};
use std::collections::HashMap;
use std::sync::OnceLock;

mod block_controller;
mod hyperlinks;
mod package_controller;
mod sections_chapter;
mod space_breaks;
pub mod tables;
mod text_alignment;
mod text_formatting;
mod text_listing;

// FUNZIONE PER I COMMAND
type BlockTranslationFn = fn(name: &str, Vec<RequiredArgNode>, Vec<OptionalArgNode>, Vec<AstItemNode>) -> String;

// FUNZIONE PER I COMMAND
type CommandTranslationFn = fn(name: &str, Vec<RequiredArgNode>, Vec<OptionalArgNode>) -> String;

// OnceLock è usato per inizializzare le mappe di traduzione una sola volta, all'avvio del programma.
// Così facendo si evita di evita di ricostruirle ad ogni invocazione di translate_block e translate_command

static BLOC_TRANS_MAP: OnceLock<HashMap<&'static str, BlockTranslationFn>> = OnceLock::new();
static COMMAND_TRANS_MAP: OnceLock<HashMap<&'static str, CommandTranslationFn>> = OnceLock::new();

// -------------------------------------------------------------------------------------------------
// --------------------------------------- HASH MAP ------------------------------------------------
// -------------------------------------------------------------------------------------------------
//TODO change strings with enums

fn get_block_trans_map() -> &'static HashMap<&'static str, BlockTranslationFn> {
    BLOC_TRANS_MAP.get_or_init(|| {
        let mut m = HashMap::new();
        // BEGIN END CONTROLLER
        m.insert(
            "begin",
            block_controller::begin_handler as BlockTranslationFn,
        );
        m.insert("end", block_controller::end_handler as BlockTranslationFn);
        m
    })
}

fn get_command_trans_map() -> &'static HashMap<&'static str, CommandTranslationFn> {
    COMMAND_TRANS_MAP.get_or_init(|| {
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
            "textcolor",
            text_formatting::render_textcolor as CommandTranslationFn,
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
        m.insert("break", space_breaks::render_space_breaks as CommandTranslationFn);
        m.insert("hfill", space_breaks::render_space_breaks as CommandTranslationFn);
        m.insert("vfill", space_breaks::render_space_breaks as CommandTranslationFn);
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
        m
    })
}

pub fn translate_block(block: &BlockNode) -> Option<String> {
    let map = get_block_trans_map();
    map.get(block.name.as_str()).map(|f| {
        f(
            &*block.name,
            block.required_args.clone(),
            block.optional_args.clone(),
            block.items.clone(),
        )
    })
}

pub fn translate_command(command: &CommandNode) -> Option<String> {
    let map = get_command_trans_map();
    map.get(command.name.as_str()).map(|f| {
        f(
            &*command.name,
            command.required_args.clone(),
            command.optional_args.clone(),
        )
    })
}

// ------------------------------------ ARGUMENT RENDERING------------------------------------------

fn out_of_bounds_reqs_arg(reqs: &[RequiredArgNode], start: usize) -> String {
    let mut extra = String::new();
    for req in reqs.iter().skip(start) {
        extra.push_str(&render_args_item(&req.items));
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
