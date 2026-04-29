use std::collections::HashMap;
use std::sync::OnceLock;

static REQARG_MAP: OnceLock<HashMap<&'static str, &'static i32>> = OnceLock::new();

pub fn reqarg_count(name: &str) -> Option<i32> {
    let map = get_reqarg_map();
    map.get(name).map(|&count| *count)
}

fn get_reqarg_map() -> &'static HashMap<&'static str, &'static i32> {
    REQARG_MAP.get_or_init(|| {
        let mut m = HashMap::new();
        // PACKAGE HANDLER
        m.insert("usepackage", &1);
        // TEXT FORMATTING
        m.insert("textbf", &1);
        m.insert("textit", &1);
        m.insert("underline", &1);
        m.insert("textcolor", &1);
        // TEXT ALIGNMENT
        m.insert("centering", &0);
        m.insert("raggedright", &0);
        m.insert("raggedleft", &0);
        m.insert("flushright", &0);
        m.insert("flushleft", &0);
        // SPACE AND BREAKS
        m.insert("newline", &0);
        m.insert("break", &0);
        m.insert("hfill", &0);
        m.insert("vfill", &0);
        m.insert("pagebreak", &0);
        m.insert("newpage", &0);
        m.insert("clearpage", &0);
        // LISTING
        m.insert("item", &0);
        // SECTION CHAPTER TITLE
        m.insert("part", &1);
        m.insert("chapter", &1);
        m.insert("section", &1);
        m.insert("subsection", &1);
        m.insert("subsubsection", &1);
        m.insert("paragraph", &1);
        m.insert("subparagraph", &1);
        m.insert("title", &1);
        m.insert("maketitle", &0);
        m.insert("author", &1);
        m.insert("date", &1);
        m.insert("today", &0);
        m.insert("tableofcontents", &0);
        m.insert("documentclass", &1);
        // HYPERLINKS
        m.insert("href", &1);
        m
    })
}
