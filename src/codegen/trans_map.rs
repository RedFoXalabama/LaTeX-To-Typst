use crate::codegen;
use crate::latex_semantic::{ArgItemNode, CommandNode, KvPairNode, OptItemNode, OptValueNode, OptionalArgNode, OptionalEntryNode, RequiredArgNode};
use std::collections::HashMap;
use std::sync::OnceLock;

mod text_formatting;
mod text_alignment;
mod begin_end_controller;
mod package_controller;
mod space_breaks;
mod text_listing;
mod sections_chapter;

// FUNZIONE PER I COMMAND
type TranslationFn = fn(name: &str, Vec<RequiredArgNode>, Vec<OptionalArgNode>) -> String;

// OnceLock inizializzarla una sola volta e renderla disponibile ovunque
static TRANS_MAP: OnceLock<HashMap<&'static str, TranslationFn>> = OnceLock::new();

// -------------------------------------------------------------------------------------------------
// --------------------------------------- HASH MAP ------------------------------------------------
// -------------------------------------------------------------------------------------------------
fn get_trans_map() -> &'static HashMap<&'static str, TranslationFn> {
    TRANS_MAP.get_or_init(|| {
        let mut m = HashMap::new();
        // PACKAGE HANDLER
        m.insert("usepackage", package_controller::package_handler as TranslationFn);
        // BEGIN HANDLER
        m.insert("begin", begin_end_controller::begin_handler as TranslationFn);
        m.insert("end", begin_end_controller::end_handler as TranslationFn);
        // TEXT FORMATTING
        m.insert("textbf", text_formatting::render_formatting as TranslationFn);
        m.insert("textit", text_formatting::render_formatting as TranslationFn);
        m.insert("underline", text_formatting::render_formatting as TranslationFn);
        m.insert("textcolor", text_formatting::render_textcolor as TranslationFn);
        // TEXT ALIGNMENT
        m.insert("centering", text_alignment::render_document_alignment as TranslationFn);
        m.insert("raggedright", text_alignment::render_document_alignment as TranslationFn);
        m.insert("raggedleft", text_alignment::render_document_alignment as TranslationFn);
        m.insert("flushright", text_alignment::render_document_alignment as TranslationFn);
        m.insert("flushleft", text_alignment::render_document_alignment as TranslationFn);
        // SPACE AND BREAKS
        m.insert("newline", space_breaks::render_space_breaks as TranslationFn);
        m.insert("break", space_breaks::render_space_breaks as TranslationFn);
        m.insert("hfill", space_breaks::render_space_breaks as TranslationFn);
        m.insert("vfill", space_breaks::render_space_breaks as TranslationFn);
        m.insert("pagebreak", space_breaks::render_space_breaks as TranslationFn);
        m.insert("newpage", space_breaks::render_space_breaks as TranslationFn);
        m.insert("clearpage", space_breaks::render_space_breaks as TranslationFn);
        // LISTING
        m.insert("item", text_listing::render_list as TranslationFn);
        // SECTION AND CHAPTER
        m.insert("part", sections_chapter::render_section_chapter as TranslationFn);
        m.insert("chapter", sections_chapter::render_section_chapter as TranslationFn);
        m.insert("section", sections_chapter::render_section_chapter as TranslationFn);
        m.insert("subsection", sections_chapter::render_section_chapter as TranslationFn);
        m.insert("subsubsection", sections_chapter::render_section_chapter as TranslationFn);
        m.insert("paragraph", sections_chapter::render_section_chapter as TranslationFn);
        m.insert("subparagraph", sections_chapter::render_section_chapter as TranslationFn);
        
        m
    })
}

pub fn translate_command(command: &CommandNode) -> Option<String> {
    let map = get_trans_map();
    map.get(command.name.as_str())
        .map(|f| f(&*command.name, command.required_args.clone(), command.optional_args.clone()))
}

// ------------------------------------ ARGUMENT RENDERING------------------------------------------

fn out_of_bounds_reqs_arg(reqs: &[RequiredArgNode], start: usize) -> String {
    let mut extra = String::new();
    for req in reqs.iter().skip(start) {
        extra.push_str(&render_args_item(&req.items));
    }
    extra
}

fn render_args_item(seq: &Vec<ArgItemNode>) -> String {
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
    out.push_str(&format!("{}={}", kv_pair.key, render_kv_value(&kv_pair.value)));
    out
}

fn render_kv_value(value: &OptValueNode) -> String {
    match value {
        OptValueNode::Simple(s) => s.clone(),
        OptValueNode::List(lst) => lst.join(","),
    }
}

fn render_opt_items(items: &Vec<OptItemNode>) -> String {
    items.into_iter()
        .map(|item| match item {
            OptItemNode::Command(cmd) => codegen::render_command(&cmd),
            OptItemNode::Group(group) => render_args_item(&group.items),
            OptItemNode::Newlines(newlines) => codegen::render_newlines(&newlines),
            OptItemNode::Text(text) => codegen::render_text(&text),
        })
        .collect()
}
