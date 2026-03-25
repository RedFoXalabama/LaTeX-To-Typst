// Il comando \\ é gestito tramite la grammatica che lo considera una regola Linebreak e non command e lo gestisce con la sua funzione
use crate::codegen::trans_map::{out_of_bounds_reqs_arg};
use crate::latex_semantic::{OptionalArgNode, RequiredArgNode};

pub fn render_space_breaks(name: &str, reqs: Vec<RequiredArgNode>, opts: Vec<OptionalArgNode>) -> String {
    let mut out = String::new();
    match name {
        "newline" | "break" => out.push_str("\\"),
        "hfill" => out.push_str("#h(1fr)"),
        "vfill" => out.push_str("#v(1fr)"),
        "pagebreak" | "newpage" | "clearpage" => out.push_str("#pagebreak()"),
        _ => out.push_str("RENDER-ERROR"),
    }

    // metto in coda gli altri elementi in modo che rispetti l'ordine dell'input
    out.push_str(&out_of_bounds_reqs_arg(&reqs, 0));
    out
}