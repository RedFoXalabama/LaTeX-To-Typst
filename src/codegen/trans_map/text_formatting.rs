use crate::codegen::trans_map::{out_of_bounds_reqs_arg, render_args_item};
use crate::latex_semantic::{OptionalArgNode, RequiredArgNode};

//--------------------------------------------------------------------------------------------------
// un unica funzione per gestire tutti i tipi di caso di formatting in base al nome del comando
//--------------------------------------------------------------------------------------------------
pub fn render_formatting(name: &str, reqs: Vec<RequiredArgNode>, opts: Vec<OptionalArgNode>) -> String {
    let mut out = String::new();
    if let Some(first) = reqs.first() {
        match name {
            "textbf" => out.push_str(&format!("*{}*", render_args_item(&first.items))),
            "textit" => out.push_str(&format!("_{}_", render_args_item(&first.items))),
            "underline" => out.push_str(&format!("#underline[{}]", render_args_item(&first.items))),
            _ => out.push_str(&render_args_item(&first.items)),
        }
    }

    // metto in coda gli altri elementi in modo che rispetti l'ordine dell'input
    out.push_str(&out_of_bounds_reqs_arg(&reqs, 1));
    out
}

// ------------------------------ SINGOLE FUNZIONI -------------------------------------------------
// DEAD CODE
pub fn render_bold(name: &str, reqs: Vec<RequiredArgNode>, opts: Vec<OptionalArgNode>) -> String {
    let mut out = String::new();
    if let Some(first) = reqs.first() {
        out.push_str(&format!("*{}*", render_args_item(&first.items)));
    }

    // metto in coda gli altri elementi in modo che rispetti l'ordine dell'input
    out.push_str(&out_of_bounds_reqs_arg(&reqs, 1));
    out
}

pub fn render_italic(name: &str, reqs: Vec<RequiredArgNode>, opts: Vec<OptionalArgNode>) -> String {
    let mut out = String::new();
    if let Some(first) = reqs.first() {
        out.push_str(&format!("_{}_", render_args_item(&first.items)));
    }

    // metto in coda gli altri elementi in modo che rispetti l'ordine dell'input
    out.push_str(&out_of_bounds_reqs_arg(&reqs, 1));
    out
}

pub fn render_underline(name: &str, reqs: Vec<RequiredArgNode>, opts: Vec<OptionalArgNode>) -> String {
    let mut out = String::new();
    if let Some(first) = reqs.first() {
        out.push_str(&format!("#underline[{}]", render_args_item(&first.items)));
    }

    // metto in coda gli altri elementi in modo che rispetti l'ordine dell'input
    out.push_str(&out_of_bounds_reqs_arg(&reqs, 1));
    out
}