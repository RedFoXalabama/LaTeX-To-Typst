use std::sync::atomic::Ordering;
use crate::codegen::trans_map::{out_of_bounds_reqs_arg, render_args_item};
use crate::globals::{add_in_listing_priority, pop_in_listing_priority, set_in_listing_value, ListType};
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

            // COMMENT (con il pacchetto verbatim in latex)
            "comment" => out.push_str("/*"),

            // LISTING
            "itemize" => { add_in_listing_priority(ListType::Itemize); },
            "enumerate" => { add_in_listing_priority(ListType::Enumerate); }
            "description" => { add_in_listing_priority(ListType::Description); }
            
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

            // COMMENT (con il pacchetto verbatim in latex)
            "comment" => out.push_str("*/"),

            // LISTING
            // il value bool per controllare in_listing é modificato solo se la lista priorità é vuota
            "itemize" | "enumerate" | "description" => { pop_in_listing_priority(); },
            
            _ => out.push_str("RENDER-ERROR"),
        }
    }

    // metto in coda gli altri elementi in modo che rispetti l'ordine dell'input
    out.push_str(&out_of_bounds_reqs_arg(&reqs, 1));
    out
}