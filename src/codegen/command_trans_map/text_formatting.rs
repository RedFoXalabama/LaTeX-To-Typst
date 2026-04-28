use crate::codegen::command_trans_map::{out_of_bounds_reqs_arg, render_args_item};
use crate::latex_semantic::{OptionalArgNode, RequiredArgNode};
use crate::utils::{drop_command_warn, COMMANDWARNING};

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
                out = drop_command_warn(COMMANDWARNING::NotImplemented(name.to_string()),
                                        Option::from(out),
                                        Option::from(name),
                                        Option::from(reqs.clone()));
            },
        }
    } else {
        match name {
            "{" | "}" | "%" | "&" | "$" | "#" | "_" => out.push_str(&format!("\\{}", name)),
            _ => out.push_str(format!("RENDER-ERROR = {}", name).as_str()),
        }
    }
    out.push_str(&out_of_bounds_reqs_arg(&reqs, 1));
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
