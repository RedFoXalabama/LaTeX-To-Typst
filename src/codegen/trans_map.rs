pub use crate::codegen::translate_items;
use crate::latex_semantic::{AstItemNode, OptionalArgNode, RequiredArgNode};

// Firma funzione per i comandi
pub type CommandTranslationFn =
    fn(name: &str, Vec<RequiredArgNode>, Vec<OptionalArgNode>) -> String;

// Firma funzione per i blocchi
pub type BlockTranslationFn =
    fn(name: &str, Vec<RequiredArgNode>, Vec<OptionalArgNode>, Vec<AstItemNode>) -> String;

pub trait TransMap<T> {
    fn translate(node: &T) -> Option<String>;
    fn is_supported(name: &str) -> bool;
}
