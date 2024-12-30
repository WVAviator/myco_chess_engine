use std::{
    collections::HashMap,
    sync::{OnceLock, RwLock},
};

static EVAL_CACHE: OnceLock<RwLock<HashMap<u64, i32>>> = OnceLock::new();

pub struct EvaluationCache {}

impl EvaluationCache {
    pub fn get(zobrist: u64) -> Option<i32> {
        EVAL_CACHE
            .get_or_init(|| RwLock::new(HashMap::new()))
            .read()
            .expect("failed to read eval cache")
            .get(&zobrist)
            .copied()
    }

    pub fn insert(zobrist: u64, eval: i32) {
        EVAL_CACHE
            .get_or_init(|| RwLock::new(HashMap::new()))
            .write()
            .expect("failed to write eval cache")
            .insert(zobrist, eval);
    }
}
