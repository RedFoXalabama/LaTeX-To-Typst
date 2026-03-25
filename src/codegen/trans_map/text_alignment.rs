use crate::codegen::trans_map::{out_of_bounds_reqs_arg};
use crate::latex_semantic::{OptionalArgNode, RequiredArgNode};

pub fn render_document_alignment(name: &str, reqs: Vec<RequiredArgNode>, _opts: Vec<OptionalArgNode>) -> String {
    let mut out = String::new();
    match name {
        "centering" | "Centering" => out.push_str("#set align(center);"),
        "raggedright" | "RaggedRight" | "flushleft" | "FlushLeft" => out.push_str("#set align(left);"), //ragged é al contrario su latex
        "raggedleft" | "RaggedLeft" | "flushright" | "FlushRight" => out.push_str("#set align(right);"), //ragged é al contrario su latex
        "justifying" => out.push_str("JUSTIFY NOT YET SUPPORTED (#set par(justify: true);)"),
        _ => out.push_str("RENDER-ERROR"),
    }


    // metto in coda gli altri elementi in modo che rispetti l'ordine dell'input
    out.push_str(&out_of_bounds_reqs_arg(&reqs, 0));
    out
}

// Justiying viene trattato diversamente da Latex e Typst, latex lo implementa solo con il pacchetto ragged2e
// invece in typst é presente, ma non si imposta come un allineamento, ma é un proprietà di tutto il testo che deve essere imposta
// all'inizio nel preambolo con #set par(justify: true)
