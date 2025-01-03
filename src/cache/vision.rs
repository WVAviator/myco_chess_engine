use std::{
    simd::Simd,
    sync::{Arc, OnceLock},
};

use super::{CacheConfiguration, ZobristMap};

type Value = Simd<u64, 8>;

static VISION_CACHE: OnceLock<Arc<ZobristMap<Value>>> = OnceLock::new();

#[inline]
fn get_vision_cache() -> &'static ZobristMap<Value> {
    VISION_CACHE
        .get_or_init(|| Arc::new(ZobristMap::default()))
        .as_ref()
}

pub struct VisionCache;

impl VisionCache {
    pub fn get(zobrist: u64) -> Option<Value> {
        get_vision_cache().get(&zobrist).map(|v| *v)
    }

    pub fn insert(zobrist: u64, vision: Value) {
        if CacheConfiguration::get().use_vision_cache {
            get_vision_cache().insert(zobrist, vision);
        }
    }
}
