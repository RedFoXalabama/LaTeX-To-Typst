use crate::codegen;
use crate::latex_semantic::{ArgItemNode, CommandNode, OptionalArgNode, RequiredArgNode};
use std::collections::HashMap;
use std::sync::OnceLock;

mod text_formatting;
mod text_alignment;
mod begin_end_controller;
mod package_controller;
mod space_breaks;

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
        m
    })
}

pub fn translate_command(command: &CommandNode) -> Option<String> {
    let map = get_trans_map();
    map.get(command.name.as_str())
        .map(|f| f(&*command.name, command.required_args.clone(), command.optional_args.clone()))
}

// -------------------------------------------------------------------------------------------------

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


