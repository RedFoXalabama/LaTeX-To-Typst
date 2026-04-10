use std::fs;
use chrono::Datelike;
use chrono::NaiveDate;
use crate::codegen::command_trans_map::{out_of_bounds_reqs_arg, render_args_item};
use crate::globals::{get_part_counter, update_part_counter};
use crate::latex_semantic::{OptionalArgNode, RequiredArgNode};


static ARTICLE_HEADER: &str = "Assets/Headers/article_header.txt";
static REPORT_HEADER: &str = "Assets/Headers/report_header.txt";
static BOOK_HEADER: &str = "Assets/Headers/book_header.txt";
static STANDARD_HEADER: &str = "Assets/Headers/standard_header.txt";

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
        "author" => out.push_str(format!("#let author = \"{}\"\n#set document(author: author)" , render_args_item(&reqs[0].items)).as_str()),
        "date" => out.push_str(&format!("#let date = {}\n#set document(date: date)", render_daytime(render_args_item(&reqs[0].items)))),
        "today" => out.push_str("datetime.today()"),
        // ASSUNZIONE: nel file di typst é presente sempre Titolo, Autore e Data
        "maketitle" => out.push_str("#set document(title: title)\n#align(center)[\n#text(3em, weight: \"bold\")[#title]\n#v(0em)\n#text(1.8em)[#author]\n#v(0em)\n#text(1.5em)[#date.display(\"[day] [month repr:long] [year]\")]\n]\n#v(2em)"),
        "tableofcontents" => out.push_str("#outline()"),

        _ => out.push_str(format!("RENDER-ERROR = {}", name).as_str()),
    }

    out.push_str(&out_of_bounds_reqs_arg(&reqs, 1));
    out
}

fn render_daytime(date: String) -> String {
    let d = date.trim();
    if d == "datetime.today()" {
        return d.to_string();
    }
    let mut normalized_date = String::new();

    let formats = [
        "%d/%m/%Y", "%Y/%m/%d", "%m/%d/%Y",
        "%d-%m-%Y", "%Y-%m-%d", "%m-%d-%Y",
        "%d.%m.%Y", "%Y.%m.%d", "%m.%d.%Y",
        "%d:%m:%Y", "%Y:%m:%d", "%m:%d:%Y",
        "%d %m %Y", "%Y %m %d", "%m %d %Y",
        "%Y%m%d", "%d%m%Y", "%m%d%Y",
        "%d %B %Y", "%B %d %Y", "%d %b %Y", "%b %d %Y",
        //AGGIUNGERE ALTRI TIPI DI DATE SE SI VUOLE
    ];

    for fmt in formats {
        if let Ok(parsed) = NaiveDate::parse_from_str(d, fmt) {
            normalized_date = format!(
                "datetime(day: {}, month: {}, year: {})",
                parsed.day(),
                parsed.month(),
                parsed.year()
            );
        }
    }

    normalized_date

}

pub fn render_doc_class(_name: &str, reqs: Vec<RequiredArgNode>, _opts: Vec<OptionalArgNode>) -> String {
    let mut out = String::new();
    let mut header_path = String::new();
    if let Some(first) = reqs.first() {
        match render_args_item(&first.items).as_str() {
            "article" => header_path = ARTICLE_HEADER.to_string(),
            "report" => header_path = REPORT_HEADER.to_string(),
            "book" => header_path = BOOK_HEADER.to_string(),

            _ => header_path = STANDARD_HEADER.to_string(),

        }
    }
    let header = fs::read_to_string(header_path).unwrap();
    out.push_str(&header);
    out.push_str("\n\n");

    // metto in coda gli altri elementi in modo che rispetti l'ordine dell'input
    out.push_str(&out_of_bounds_reqs_arg(&reqs, 1));
    out
}