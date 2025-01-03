use std::sync::{Arc, OnceLock};

use super::{CacheConfiguration, ZobristMap};

type Value = i32;

static EVAL_CACHE: OnceLock<Arc<ZobristMap<Value>>> = OnceLock::new();

#[inline]
fn get_eval_cache() -> &'static ZobristMap<Value> {
    EVAL_CACHE
        .get_or_init(|| Arc::new(ZobristMap::default()))
        .as_ref()
}

pub struct EvaluationCache;

impl EvaluationCache {
    pub fn get(zobrist: u64) -> Option<Value> {
        get_eval_cache().get(&zobrist).map(|v| *v)
    }

    pub fn insert(zobrist: u64, eval: Value) {
        if CacheConfiguration::get().use_eval_cache {
            get_eval_cache().insert(zobrist, eval);
        }
    }
}
