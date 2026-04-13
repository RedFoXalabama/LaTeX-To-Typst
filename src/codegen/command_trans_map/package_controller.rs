use crate::codegen::command_trans_map::{out_of_bounds_reqs_arg, render_args_item};
use crate::latex_semantic::{OptionalArgNode, RequiredArgNode};

//---------------------------------------- PACKAGE HANDLER -----------------------------------------
pub fn package_handler(_: &str, reqs: Vec<RequiredArgNode>, _opts: Vec<OptionalArgNode>) -> String {
    let mut out = String::new();
    if let Some(first) = reqs.first() {
        let req_arg = render_args_item(&first.items);
        match req_arg.as_str() {
            "ragged2e" => out.push_str("/* usepackage{ragged2e} */"),
            "verbatim" => out.push_str("/* usepackage{verbatim} */"),
            "hyperref" => out.push_str("/* usepackage{hyperref} */"),

            _ => out.push_str(format!("/*RENDER-ERROR = {}*/", "usepackage").as_str()),
        }
    }
    out.push_str(&out_of_bounds_reqs_arg(&reqs, 1));
    out
}
