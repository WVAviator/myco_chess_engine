use std::sync::{OnceLock, RwLock};

use nohash_hasher::IntMap;

static EVAL_CACHE: OnceLock<RwLock<IntMap<u64, i32>>> = OnceLock::new();

pub struct EvaluationCache {}

impl EvaluationCache {
    pub fn get(zobrist: u64) -> Option<i32> {
        EVAL_CACHE
            .get_or_init(|| RwLock::new(IntMap::default()))
            .read()
            .expect("failed to read eval cache")
            .get(&zobrist)
            .copied()
    }

    pub fn insert(zobrist: u64, eval: i32) {
        EVAL_CACHE
            .get_or_init(|| RwLock::new(IntMap::default()))
            .write()
            .expect("failed to write eval cache")
            .insert(zobrist, eval);
    }
}
