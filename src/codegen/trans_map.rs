use crate::codegen;
use crate::latex_semantic::{ArgItemNode, CommandNode, OptionalArgNode, RequiredArgNode};
use std::collections::HashMap;
use std::sync::OnceLock;

mod text_formatting;
mod text_alignment;

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
        m.insert("usepackage", package_handler as TranslationFn);
        // BEGIN HANDLER
        m.insert("begin", begin_handler as TranslationFn);
        m.insert("end", end_handler as TranslationFn);
        // TEXT FORMATTING
        m.insert("textbf", text_formatting::render_formatting as TranslationFn);
        m.insert("textit", text_formatting::render_formatting as TranslationFn);
        m.insert("underline", text_formatting::render_formatting as TranslationFn);
        // TEXT ALIGNMENT
        m.insert("centering", text_alignment::render_document_alignment as TranslationFn);
        m.insert("raggedright", text_alignment::render_document_alignment as TranslationFn);
        m.insert("raggedleft", text_alignment::render_document_alignment as TranslationFn);
        m.insert("flushright", text_alignment::render_document_alignment as TranslationFn);
        m.insert("flushleft", text_alignment::render_document_alignment as TranslationFn);

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
            ArgItemNode::Newlines(newlines) => "\n".repeat(newlines.count),
            ArgItemNode::Text(text) => codegen::render_text(&text),
        })
        .collect()
}

//---------------------------------------- PACKAGE HANDLER -----------------------------------------
fn package_handler(_name: &str, reqs: Vec<RequiredArgNode>, _opts: Vec<OptionalArgNode>) -> String {
    let mut out = String::new();
    if let Some(first) = reqs.first() {
        let req_arg = render_args_item(&first.items);
        match req_arg.as_str() {
            "ragged2e" => out.push_str("/* usepackage{ragged2e} */"),
            _ => out.push_str("RENDER-ERROR"),
        }
    }
    out.push_str(&out_of_bounds_reqs_arg(&reqs, 1));
    out
}

// ------------------------------------ BEING HANDLER ----------------------------------------------
fn begin_handler(name: &str, reqs: Vec<RequiredArgNode>, _opts: Vec<OptionalArgNode>) -> String {
    let mut out = String::new();
    if let Some(first) = reqs.first() {
        let req_arg = render_args_item(&first.items);
        match req_arg.as_str() {
            // TEXT ALIGNMENT
            "center" | "Center" => out.push_str("#align(center)["),
            "flushright" | "FlushRight" => out.push_str("#align(right)["),
            "flushleft" | "FlushLeft" => out.push_str("#align(left)["),

            _ => out.push_str("RENDER-ERROR"),
        }
    }

    // metto in coda gli altri elementi in modo che rispetti l'ordine dell'input
    out.push_str(&out_of_bounds_reqs_arg(&reqs, 1));
    out
}

fn end_handler(name: &str, reqs: Vec<RequiredArgNode>, _opts: Vec<OptionalArgNode>) -> String {
    let mut out = String::new();
    if let Some(first) = reqs.first() {
        let req_arg = render_args_item(&first.items);
        match req_arg.as_str() {
            // TEXT ALIGNMENT
            "center" | "Center" |
            "flushright" | "FlushRight" |
            "flushleft" | "FlushLeft" => out.push_str("]"),

            _ => out.push_str("RENDER-ERROR"),
        }
    }

    // metto in coda gli altri elementi in modo che rispetti l'ordine dell'input
    out.push_str(&out_of_bounds_reqs_arg(&reqs, 1));
    out
}