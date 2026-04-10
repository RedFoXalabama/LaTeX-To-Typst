pub mod block_controller;
pub mod table_controller;

use crate::codegen::trans_map::{BlockTranslationFn, TransMap};
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
            block_controller::render_center_block as BlockTranslationFn,
        );
        m.insert(
            "flushright",
            block_controller::render_flushright_block as BlockTranslationFn,
        );
        m.insert(
            "flushleft",
            block_controller::render_flushleft_block as BlockTranslationFn,
        );
        m.insert(
            "comment",
            block_controller::render_comment_block as BlockTranslationFn,
        );
        m.insert(
            "itemize",
            block_controller::render_itemize_block as BlockTranslationFn,
        );
        m.insert(
            "enumerate",
            block_controller::render_enumerate_block as BlockTranslationFn,
        );
        m.insert(
            "description",
            block_controller::render_description_block as BlockTranslationFn,
        );
        m.insert(
            "document",
            block_controller::render_document_block as BlockTranslationFn,
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
