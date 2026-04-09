pub mod table_controller;

use std::collections::HashMap;
use std::sync::OnceLock;
use crate::codegen::trans_map::{BlockTranslationFn, TransMap};
use crate::latex_semantic::BlockNode;

pub struct BlockTransMap;

static TRANS_MAP: OnceLock<HashMap<&'static str, BlockTranslationFn>> = OnceLock::new();

fn get_trans_map() -> &'static HashMap<&'static str, BlockTranslationFn> {
    TRANS_MAP.get_or_init(|| {
        let mut m = HashMap::new();
        // BEGIN END CONTROLLER
        m.insert("tabular", table_controller::render_table as BlockTranslationFn);
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
