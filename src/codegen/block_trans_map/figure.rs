use log::warn;
use crate::codegen::command_trans_map::render_args_item;
use crate::latex_semantic::{AstItemNode, OptItemNode, OptValueNode, OptionalArgNode, OptionalEntryNode, RequiredArgNode};
use crate::utils::{drop_command_warn, COMMANDWARNING};

pub fn render_figure(
    _name: &str,
    _required_args: Vec<RequiredArgNode>,
    optional_args: Vec<OptionalArgNode>,
    items: Vec<AstItemNode>,
) -> String {
    let mut out = String::new();

    let mut image_path: Option<String> = None;
    let mut width: Option<String> = None;
    let mut caption: Option<String> = None;
    let mut label: Option<String> = None;

    let mut i = 0;
    while i < items.len() {
        if let AstItemNode::Command(cmd) = &items[i] {
            match cmd.name.as_str() {
                "includegraphics" => {
                    if width.is_none() {
                        width = extract_width(&cmd.optional_args);
                    }

                    if image_path.is_none() {
                        image_path = first_required_arg_text(&cmd.required_args);
                    }

                }
                "caption" => {
                    if caption.is_none() {
                        caption = first_required_arg_text(&cmd.required_args);
                    }
                }
                "label" => {
                    if label.is_none() {
                        label = first_required_arg_text(&cmd.required_args);
                    }
                }
                "centering" => {
                    //non fare nulla perché in typst le immagini sono centrate di default, mentre in latex serve il comando per centrarle
                }
                _ => {
                    out = drop_command_warn(COMMANDWARNING::NotImplemented(cmd.name.to_string()), 
                                            Option::from(out), Option::from(cmd.name.as_str()), 
                                            Option::from(cmd.required_args.clone()));
                }
            }
        }
        i += 1;
    }

    let path = image_path.unwrap_or_else(|| "Latex2Typst.png".to_string());
    let cap = caption.unwrap_or_else(|| "Qui va la tua didascalia".to_string());

    let width_part = width
        .map(|w| format!(", width: {}", w))
        .unwrap_or_default();

    let label_part = label
        .map(|l| normalize_label(&l))
        .filter(|l| !l.is_empty())
        .map(|l| format!(" <{}>", l))
        .unwrap_or_default();

    let placement_part = figure_placement(&optional_args)
        .map(|p| format!("\n  placement: {},", p))
        .unwrap_or_default();

    out.push_str(format!(
        "#context figure(\n  image(\"{}\"{}),{}\n  caption: [{}],\n){}",
        escape_typst_string(&path),
        width_part,
        placement_part,
        cap,
        label_part
    ).as_str());
    out
}

fn first_required_arg_text(reqs: &[RequiredArgNode]) -> Option<String> {
    reqs.first()
        .map(|r| render_args_item(&r.items).trim().to_string())
        .filter(|s| !s.is_empty())
}

fn extract_width(opts: &[OptionalArgNode]) -> Option<String> {
    for opt in opts {
        for entry in &opt.entries {
            if let OptionalEntryNode::KeyValue(kv) = entry {
                if kv.key.trim() == "width" {
                    return Some(latex_width_to_typst(&opt_value_to_string(&kv.value)));
                }
            }
        }
    }
    None
}

fn opt_value_to_string(v: &OptValueNode) -> String {
    match v {
        OptValueNode::Simple(s) => s.trim().to_string(),
        OptValueNode::List(lst) => lst.join(","),
    }
}

fn normalize_label(label: &str) -> String {
    label
        .trim()
        .trim_start_matches('{')
        .trim_end_matches('}')
        .replace(':', "-")
}

fn escape_typst_string(s: &str) -> String {
    s.replace('"', "\\\"")
}

// ------------------------------- GESTIONE POSIZIONAMENTO -----------------------------------------
fn figure_placement(optional_args: &[OptionalArgNode]) -> Option<String> {
    for opt in optional_args {
        for entry in &opt.entries {
            match entry {
                OptionalEntryNode::Items(items) => {
                    let raw = items.iter().map(|item| match item {
                        OptItemNode::Text(t) => t.value.as_str(),
                        _ => "",
                    }).collect::<String>();

                    if let Some(mapped) = map_placement_spec(&raw) {
                        return Some(mapped);
                    }
                }
                OptionalEntryNode::KeyValue(kv) if kv.key.trim() == "placement" => {
                    let raw = opt_value_to_string(&kv.value);
                    if let Some(mapped) = map_placement_spec(&raw) {
                        return Some(mapped);
                    }
                }
                _ => {}
            }
        }
    }

    None
}

fn map_placement_spec(raw: &str) -> Option<String> {
    let spec: String = raw
        .chars()
        .filter(|c| !c.is_whitespace() && *c != '!')
        .collect::<String>()
        .to_lowercase();

    if spec.is_empty() {
        return None;
    }

    if spec.contains('h') {
        return Some("none".to_string());
    }

    if spec.contains('t') {
        return Some("top".to_string());
    }

    if spec.contains('b') {
        return Some("bottom".to_string());
    }

    if spec.contains('p') {
        warn!("LaTeX usa [p] per una pagina di float: in Typst non c'è un equivalente diretto, uso `none`.");
        return Some("none".to_string());
    }

    None
}

// ------------------------------- GESTIONE CASI SPAZIATURA IMMAGINI -------------------------------
fn latex_width_to_typst(raw: &str) -> String {
    let compact: String = raw.chars().filter(|c| !c.is_whitespace()).collect();
    if compact.is_empty() {
        return compact;
    }

    // Caso: solo macro, es. \linewidth
    if let Some(mapped) = map_latex_length_macro(1.0, &compact) {
        return mapped;
    }

    // Caso: fattore * macro, es. 0.5\linewidth
    if let Some((factor, macro_name)) = split_number_macro(&compact) {
        if let Some(mapped) = map_latex_length_macro(factor, macro_name) {
            return mapped;
        }
    }

    // Caso: numero + unità assoluta, es. 12pt, 3.5cm
    if let Some((num, unit)) = split_number_unit(&compact) {
        return match unit {
            "pt" | "mm" | "cm" | "in" | "em" => format!("{}{}", trim_float(num), unit),
            // Typst non ha `ex` nativo: approssimazione comune 1ex ~= 0.5em
            "ex" => format!("{}em", trim_float(num * 0.5)),
            _ => compact,
        };
    }

    // Fallback: lascia invariato
    compact
}

fn split_number_macro(s: &str) -> Option<(f64, &str)> {
    let idx = s.find('\\')?;
    if idx == 0 {
        return Some((1.0, s));
    }
    let (num_part, macro_part) = s.split_at(idx);
    let factor = num_part.parse::<f64>().ok()?;
    Some((factor, macro_part))
}

fn split_number_unit(s: &str) -> Option<(f64, &'static str)> {
    // Ordine importante: unità di 2 caratteri tutte uguali qui, ma resta esplicito.
    const UNITS: [&str; 6] = ["pt", "mm", "cm", "in", "ex", "em"];

    for unit in UNITS {
        if let Some(num_part) = s.strip_suffix(unit) {
            if let Ok(num) = num_part.parse::<f64>() {
                return Some((num, unit));
            }
        }
    }
    None
}

fn map_latex_length_macro(factor: f64, macro_name: &str) -> Option<String> {
    match macro_name {
        // Relative al contenitore corrente: in Typst rendono bene come percentuale
        "\\columnwidth" | "\\linewidth" | "\\textwidth" => {
            Some(format!("{}%", trim_float(factor * 100.0)))
        }

        // Dimensioni pagina Typst
        "\\paperwidth" => Some(scale_expr(factor, "page.width")),
        "\\paperheight" => Some(scale_expr(factor, "page.height")),
        "\\textheight" => Some(scale_expr(factor, "page.height")),
        "\\columnsep" => Some(format!("{}em", trim_float(factor))),
        "\\unitlength" => Some(format!("{}pt", trim_float(factor))),

        _ => None,
    }
}

fn scale_expr(factor: f64, base: &str) -> String {
    if (factor - 1.0).abs() < f64::EPSILON {
        base.to_string()
    } else {
        format!("{} * {}", trim_float(factor), base)
    }
}

fn trim_float(v: f64) -> String {
    if v.fract().abs() < f64::EPSILON {
        format!("{}", v as i64)
    } else {
        let s = format!("{:.6}", v);
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    }
}