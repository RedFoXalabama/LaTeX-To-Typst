use crate::codegen;
use crate::latex_semantic::{ArgItemNode, CommandNode, OptionalArgNode, RequiredArgNode, TextNode};
use std::collections::HashMap;
use std::sync::OnceLock;

// FUNZIONE PER I COMMAND
type TranslationFn = fn(Vec<RequiredArgNode>, Vec<OptionalArgNode>) -> String;

// OnceLock inizializzarla una sola volta e renderla disponibile ovunque
static TRANS_MAP: OnceLock<HashMap<&'static str, TranslationFn>> = OnceLock::new();

// -------------------------------------------------------------------------------------------------
// --------------------------------------- HASH MAP ------------------------------------------------
// -------------------------------------------------------------------------------------------------
fn get_trans_map() -> &'static HashMap<&'static str, TranslationFn> {
    TRANS_MAP.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert("textbf", render_bold as TranslationFn);
        m
    })
}

pub fn translate_command(command: &CommandNode) -> Option<String> {
    let map = get_trans_map();
    map.get(command.name.as_str())
        .map(|f| f(command.required_args.clone(), command.optional_args.clone()))
}

// ------------------------------------- UTILS FUNCTION --------------------------------------------

fn out_of_bounds_reqs_arg(reqs: &[RequiredArgNode], start: usize) -> String {
    let mut extra = String::new();
    for req in reqs.iter().skip(start) {
        extra.push_str(&render_sequence(&req.items));
    }
    extra
}

// ------------------------------- TRANSLATION FUNCTION --------------------------------------------
fn render_bold(reqs: Vec<RequiredArgNode>, opts: Vec<OptionalArgNode>) -> String {
    let mut out = String::new();
    if let Some(first) = reqs.first() {
        out.push_str(&format!("*{}*", render_sequence(&first.items)));
    }

    // metto in coda gli altri elementi in modo che rispetti l'ordine dell'input
    out.push_str(&out_of_bounds_reqs_arg(&reqs, 1));
    out
}

fn render_sequence(seq: &Vec<ArgItemNode>) -> String {
    seq.into_iter()
        .map(|item| match item {
            ArgItemNode::Command(cmd) => codegen::render_command(&cmd),
            ArgItemNode::Group(group) => render_sequence(&group.items),
            ArgItemNode::Newlines(newlines) => "\n".repeat(newlines.count),
            ArgItemNode::Text(text) => codegen::render_text(&text),
        })
        .collect()
}
