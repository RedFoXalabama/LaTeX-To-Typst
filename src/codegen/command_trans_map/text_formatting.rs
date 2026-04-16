use log::warn;
use crate::codegen::command_trans_map::{out_of_bounds_reqs_arg, render_args_item};
use crate::latex_semantic::{OptionalArgNode, RequiredArgNode};

//--------------------------------------------------------------------------------------------------
// un unica funzione per gestire tutti i tipi di caso di formatting in base al nome del comando
//--------------------------------------------------------------------------------------------------
pub fn render_formatting(name: &str, reqs: Vec<RequiredArgNode>, _opts: Vec<OptionalArgNode>) -> String {
    let mut out = String::new();
    if let Some(first) = reqs.first() {
        match name {
            "textbf" => out.push_str(&format!("*{}*", render_args_item(&first.items))),
            "textit" => out.push_str(&format!("_{}_", render_args_item(&first.items))),
            "underline" => out.push_str(&format!("#underline[{}]", render_args_item(&first.items))),

            _ => {
                let error_msg = format!("ERROR: NOT-YET-IMPLEMENTED \\{}{{{}}}", name, reqs.iter().map(|r| render_args_item(&r.items)).collect::<Vec<_>>().join("}{"));
                warn!("==> {}", error_msg);
                out.push_str(format!("/*{}*/",error_msg).as_str());
            },
        }
    }

    out
}

pub fn render_textcolor(_name: &str, reqs: Vec<RequiredArgNode>, _opts: Vec<OptionalArgNode>) -> String {
    let mut out = String::new();
    if reqs.len() >= 2 {
        let color_arg = &reqs[0];
        let text_arg = &reqs[1];
        out.push_str(&format!("#text({})[{}]", render_args_item(&color_arg.items), render_args_item(&text_arg.items)));
    }

    // metto in coda gli altri elementi in modo che rispetti l'ordine dell'input
    out.push_str(&out_of_bounds_reqs_arg(&reqs, 2));
    out
}
