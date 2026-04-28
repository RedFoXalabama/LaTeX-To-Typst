use crate::codegen::command_trans_map::{out_of_bounds_reqs_arg, render_args_item};
use crate::latex_semantic::{OptionalArgNode, RequiredArgNode};
use crate::utils::{COMMANDWARNING, drop_command_warn};

//---------------------------------------- PACKAGE HANDLER -----------------------------------------
pub fn package_handler(
    name: &str,
    reqs: Vec<RequiredArgNode>,
    _opts: Vec<OptionalArgNode>,
) -> String {
    let mut out = String::new();
    if let Some(first) = reqs.first() {
        let req_arg = render_args_item(&first.items);
        match req_arg.as_str() {
            "ragged2e" | "verbatim" | "listing" | "hyperref" | "listings" | "graphicx" => {
                out.push_str(&format!("/* usepackage{{{req_arg}}} */"));
            }

            _ => {
                out = drop_command_warn(
                    COMMANDWARNING::NotImplemented(name.to_string()),
                    Option::from(out),
                    Option::from(name),
                    Option::from(reqs.clone()),
                );
            }
        }
    }
    out.push_str(&out_of_bounds_reqs_arg(&reqs, 1));
    out
}
