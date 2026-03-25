use crate::codegen::trans_map::{out_of_bounds_reqs_arg, render_args_item};
use crate::globals::{get_part_counter, update_part_counter, PART_COUNTER};
use crate::latex_semantic::{OptionalArgNode, RequiredArgNode};

pub fn render_section_chapter(name: &str, reqs: Vec<RequiredArgNode>, opts: Vec<OptionalArgNode>) -> String {
    let mut out = String::new();
    match name {
        "part" => {
            update_part_counter();
            out.push_str(format!("#v(2em)\n#align(center)[\n#text(1.2em)[Part {}]\n#v(0.5em)\n#text(2em, weight: \"bold\")[{}]\n]\n#v(2em)\n", get_part_counter(), render_args_item(&reqs[0].items)).as_str())
        }
        "chapter" => out.push_str(format!("= {}\n", render_args_item(&reqs[0].items)).as_str()),
        "section" => out.push_str(format!("== {}\n", render_args_item(&reqs[0].items)).as_str()),
        "subsection" => out.push_str(format!("=== {}\n", render_args_item(&reqs[0].items)).as_str()),
        "subsubsection" => out.push_str(format!("==== {}\n", render_args_item(&reqs[0].items)).as_str()),
        "paragraph" => out.push_str(format!("===== {}\n", render_args_item(&reqs[0].items)).as_str()),
        "subparagraph" => out.push_str(format!("====== {}\n", render_args_item(&reqs[0].items)).as_str()),


        _ => out.push_str("RENDER-ERROR"),
    }
    out.push_str(&out_of_bounds_reqs_arg(&reqs, 1));
    out
}