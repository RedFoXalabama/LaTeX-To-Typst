use crate::codegen::translate_items;
use crate::latex_semantic::{AstItemNode, OptionalArgNode, RequiredArgNode};

pub fn render_verbatim(
    _name: &str,
    _required_args: Vec<RequiredArgNode>,
    _optional_args: Vec<OptionalArgNode>,
    items: Vec<AstItemNode>,
) -> String {
    let mut out = String::new();
    out.push_str("```");
    out.push_str(&translate_items(items));
    out.push_str("```");
    out
}

pub fn render_lstlisting(
    _name: &str,
    _required_args: Vec<RequiredArgNode>,
    _optional_args: Vec<OptionalArgNode>,
    items: Vec<AstItemNode>,
) -> String {
    let mut out = String::new();
    let language = _optional_args
        .first()
        .and_then(|opt| {
            opt.entries.iter().find_map(|entry| {
                if let crate::latex_semantic::OptionalEntryNode::KeyValue(kv) = entry {
                    if kv.key == "language" {
                        if let crate::latex_semantic::OptValueNode::Simple(s) = &kv.value {
                            return Some(s.to_lowercase());
                        }
                    }
                }
                None
            })
        })
        .unwrap_or_default();

    out.push_str(format!("```{}", language).as_str());
    out.push_str(&translate_items(items));
    out.push_str("```");
    out
}
