use crate::codegen::trans_map::{out_of_bounds_reqs_arg, render_args_item};
use crate::latex_semantic::{OptionalArgNode, RequiredArgNode};

pub fn render_href(_name: &str, reqs: Vec<RequiredArgNode>, _opts: Vec<OptionalArgNode>) -> String {
    let mut out = String::new();
    if let Some(first) = reqs.first() {
        let url = render_args_item(&first.items);
        if let Some(second) = reqs.get(1) {
            let text = render_args_item(&second.items);
            out.push_str(format!("#link(\"{}\")[{}]", url, text).as_str());
        } else {
            out.push_str(format!("#link(\"{}\")", url).as_str());
        }
    }
    out.push_str(&out_of_bounds_reqs_arg(&reqs, 2));
    out
}