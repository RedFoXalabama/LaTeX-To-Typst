use crate::codegen::trans_map::{out_of_bounds_reqs_arg, render_args_item};
use crate::latex_semantic::{OptionalArgNode, RequiredArgNode};

pub fn begin_handler(name: &str, reqs: Vec<RequiredArgNode>, _opts: Vec<OptionalArgNode>) -> String {
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

pub fn end_handler(name: &str, reqs: Vec<RequiredArgNode>, _opts: Vec<OptionalArgNode>) -> String {
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