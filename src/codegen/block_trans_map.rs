pub mod alignment;
mod code;
pub mod comments;
pub mod listings;
pub mod table_controller;
mod figure;

use crate::codegen::trans_map::{BlockTranslationFn, TransMap};
use crate::codegen::translate_items;
use crate::latex_semantic::BlockNode;
use std::collections::HashMap;
use std::sync::OnceLock;

pub struct BlockTransMap;

static TRANS_MAP: OnceLock<HashMap<&'static str, BlockTranslationFn>> = OnceLock::new();

fn get_trans_map() -> &'static HashMap<&'static str, BlockTranslationFn> {
    TRANS_MAP.get_or_init(|| {
        let mut m = HashMap::new();
        // BEGIN END CONTROLLER
        m.insert(
            "tabular",
            table_controller::render_table as BlockTranslationFn,
        );
        m.insert(
            "center",
            alignment::render_center_block as BlockTranslationFn,
        );
        m.insert(
            "flushright",
            alignment::render_flushright_block as BlockTranslationFn,
        );
        m.insert(
            "flushleft",
            alignment::render_flushleft_block as BlockTranslationFn,
        );
        m.insert(
            "comment",
            comments::render_comment_block as BlockTranslationFn,
        );
        m.insert(
            "itemize",
            listings::render_itemize_block as BlockTranslationFn,
        );
        m.insert(
            "enumerate",
            listings::render_enumerate_block as BlockTranslationFn,
        );
        m.insert(
            "description",
            listings::render_description_block as BlockTranslationFn,
        );
        m.insert("verbatim", code::render_verbatim as BlockTranslationFn);
        m.insert("lstlisting", code::render_lstlisting as BlockTranslationFn);
        m.insert("document", |_, _, _, items| translate_items(items));
        // IMMAGINI
        m.insert(
            "figure",
            figure::render_figure as BlockTranslationFn,
        );
        m
    })
}

impl TransMap<BlockNode> for BlockTransMap {
    fn translate(block: &BlockNode) -> Option<String> {
        let map = get_trans_map();
        map.get(block.name.as_str()).map(|f| {
            f(
                &*block.name,
                block.required_args.clone(),
                block.optional_args.clone(),
                block.items.clone(),
            )
        })
    }
}
