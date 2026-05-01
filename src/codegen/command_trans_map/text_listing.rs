use crate::codegen::command_trans_map::{out_of_bounds_reqs_arg, render_opt_entry};
use crate::globals::{get_in_listing_priority, read_in_listing_priority, ListType};
use crate::latex_semantic::{OptionalArgNode, RequiredArgNode};
use crate::utils::{drop_command_warn, COMMANDWARNING};

pub fn render_list(name: &str, reqs: Vec<RequiredArgNode>, opts: Vec<OptionalArgNode>) -> String {
    let mut out = String::new();
    match name {
        "item" => {
            match read_in_listing_priority() {
                Some(ListType::Itemize) => {
                    render_list_line("- ".to_string(), &mut out, opts);
                }
                Some(ListType::Enumerate) => {
                    render_list_line("+ ".to_string(), &mut out, opts);
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
                    out = drop_command_warn(COMMANDWARNING::NotImplemented(name.to_string()),
                                            Option::from(out),
                                            Option::from(name),
                                            Option::from(reqs.clone()));
                },
            }
        },

        _ => {
            out = drop_command_warn(COMMANDWARNING::NotImplemented(name.to_string()),
                                    Option::from(out),
                                    Option::from(name),
                                    Option::from(reqs.clone()));
        },
    }
    out_of_bounds_reqs_arg(&reqs, 0);
    out
}

fn render_list_line(sign: String, out: &mut String, opts: Vec<OptionalArgNode>) -> &mut String {
    out.push('\n');
    out.push_str(&"\t".repeat(get_in_listing_priority().len() - 1));
    out.push_str(&*sign);
    if let Some(first) = opts.first() {
        let opts_arg = render_opt_entry(&first.entries);
        if !opts_arg.is_empty() {
            out.push_str(&format!(" [{}]", opts_arg));
        }
    }
    out
}