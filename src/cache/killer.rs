use std::{
    collections::VecDeque,
    sync::{OnceLock, RwLock},
};

use array_init::array_init;

use crate::moves::simple_move::SimpleMove;

const MAX_KILLERS_PER_DEPTH: usize = 3;

const KILLERS_CACHE: OnceLock<RwLock<[VecDeque<SimpleMove>; 32]>> = OnceLock::new();

pub struct KillerCache;

impl KillerCache {
    pub fn is_killer(depth: usize, lmove: &SimpleMove) -> bool {
        KILLERS_CACHE
            .get_or_init(|| RwLock::new(array_init(|_| VecDeque::new())))
            .read()
            .unwrap()[depth & 31]
            .contains(lmove)
    }

    pub fn add_killer(depth: usize, lmove: &SimpleMove) {
        let cache = KILLERS_CACHE;
        let cache_lock = cache.get_or_init(|| RwLock::new(array_init(|_| VecDeque::new())));
        let mut cache = cache_lock.write().unwrap();

        let depth = depth & 31;
        cache[depth].push_back(*lmove);
        if cache[depth].len() > MAX_KILLERS_PER_DEPTH {
            cache[depth].pop_front();
        }
    }
}
