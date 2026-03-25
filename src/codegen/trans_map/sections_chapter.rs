use crate::codegen::trans_map::{out_of_bounds_reqs_arg, render_args_item};
use crate::globals::{get_part_counter, update_part_counter};
use crate::latex_semantic::{OptionalArgNode, RequiredArgNode};

pub fn render_section_chapter(name: &str, reqs: Vec<RequiredArgNode>, _opts: Vec<OptionalArgNode>) -> String {
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


        _ => out.push_str(format!("RENDER-ERROR = {}", name).as_str()),
    }
    out.push_str(&out_of_bounds_reqs_arg(&reqs, 1));
    out
}

pub fn render_info_document(name: &str, reqs: Vec<RequiredArgNode>, _opts: Vec<OptionalArgNode>) -> String {
    let mut out = String::new();
    match name {
        "title" => out.push_str(format!("#let title = [{}]", render_args_item(&reqs[0].items)).as_str()),
        "maketitle" => out.push_str("#set document(title: title)\n#align(center)[\n#text(2em, weight: \"bold\")[#title]\n]"),

        "author" => out.push_str(format!("#let author = \"{}\"\n#set document(author: author)" , render_args_item(&reqs[0].items)).as_str()),
        "date" => out.push_str(&format!("#let date = {}\n#set document(date: date)", render_daytime(render_args_item(&reqs[0].items)))),
        "today" => out.push_str("datetime.today()"),
        "maketitle" => todo!(),
        "tableofcontents" => todo!(),

        _ => out.push_str(format!("RENDER-ERROR = {}", name).as_str()),
    }

    out.push_str(&out_of_bounds_reqs_arg(&reqs, 1));
    out
}

fn render_daytime(date: String) -> String {
    if date == "datetime.today()" {
        return date
    }

    // let accepted = vec![
    //     "03/31/2014",
    // ];
    // for date_str in accepted {
    //     let result = date_str.parse::<DateTimeUtc>();
    //     print!("Parsing date string '{:?}': ", result);
    //     assert!(result.is_ok())
    // }

    let mut parts = date.split('/');

    let part1 = parts.next().unwrap_or_default().to_string();
    let part2 = parts.next().unwrap_or_default().to_string();
    let part3 = parts.next().unwrap_or_default().to_string();

    format!("datetime(day: {}, month: {}, year: {})", part1, part2, part3)

}