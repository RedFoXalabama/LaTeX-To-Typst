use log::warn;
use crate::codegen::command_trans_map::{out_of_bounds_reqs_arg, render_args_item, render_opt_entry};
use crate::globals::{get_in_listing_priority, read_in_listing_priority, ListType};
use crate::latex_semantic::{OptionalArgNode, RequiredArgNode};

pub fn render_list(name: &str, reqs: Vec<RequiredArgNode>, opts: Vec<OptionalArgNode>) -> String {
    let mut out = String::new();
    match name {
        "item" => {
            match read_in_listing_priority() {
                Some(ListType::Itemize) => {
                    render_list_line('-', &mut out, opts);
                }
                Some(ListType::Enumerate) => {
                    render_list_line('+', &mut out, opts);
                }
                Some(ListType::Description) => {
                    out.push('\n');
                    out.push_str(&"\t".repeat(get_in_listing_priority().len() - 1));
                    out.push('/');
                    if let Some(first) = opts.first() {
                        let opts_arg = render_opt_entry(&first.entries);
                        if !opts_arg.is_empty() {
                            out.push_str(&format!(" {}:", opts_arg));
                        }
                    }
                },
                None => {
                    let error_msg = format!("ERROR: NOT-YET-IMPLEMENTED \\{}{{{}}}", name, reqs.iter().map(|r| render_args_item(&r.items)).collect::<Vec<_>>().join("}{"));
                    warn!("==> {}", error_msg);
                    out.push_str(format!("/*{}*/",error_msg).as_str());
                },
            }
        },

        _ => {
            let error_msg = format!("ERROR: NOT-YET-IMPLEMENTED \\{}{{{}}}", name, reqs.iter().map(|r| render_args_item(&r.items)).collect::<Vec<_>>().join("}{"));
            warn!("==> {}", error_msg);
            out.push_str(format!("/*{}*/",error_msg).as_str());
        },
    }
    out_of_bounds_reqs_arg(&reqs, 0);
    out
}

fn render_list_line(sign: char, out: &mut String, opts: Vec<OptionalArgNode>) -> &mut String {
    out.push('\n');
    out.push_str(&"\t".repeat(get_in_listing_priority().len() - 1));
    out.push(sign);
    if let Some(first) = opts.first() {
        let opts_arg = render_opt_entry(&first.entries);
        if !opts_arg.is_empty() {
            out.push_str(&format!(" [{}]", opts_arg));
        }
    }
    out
}