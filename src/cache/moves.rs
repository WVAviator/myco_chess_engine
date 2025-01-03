use std::sync::{Arc, OnceLock};

use arrayvec::ArrayVec;

use crate::moves::simple_move::SimpleMove;

use super::{CacheConfiguration, ZobristMap};

type Value = ArrayVec<SimpleMove, 256>;

static MOVES_CACHE: OnceLock<Arc<ZobristMap<Value>>> = OnceLock::new();

#[inline]
fn get_moves_cache() -> &'static ZobristMap<Value> {
    MOVES_CACHE
        .get_or_init(|| Arc::new(ZobristMap::default()))
        .as_ref()
}

pub struct MovesCache;

impl MovesCache {
    pub fn get(zobrist: u64) -> Option<Value> {
        get_moves_cache().get(&zobrist).map(|v| v.clone())
    }

    pub fn insert(zobrist: u64, moves: Value) {
        if CacheConfiguration::get().use_eval_cache {
            get_moves_cache().insert(zobrist, moves);
        }
    }
}
