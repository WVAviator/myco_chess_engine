use std::sync::OnceLock;

use dashmap::DashMap;
use nohash_hasher::BuildNoHashHasher;

pub mod eval;
pub mod moves;
pub mod vision;

static GLOBAL_CACHE_CONFIG: OnceLock<CacheConfiguration> = OnceLock::new();

pub type ZobristMap<V> = DashMap<u64, V, BuildNoHashHasher<u64>>;

#[derive(Debug, Clone, PartialEq)]
pub struct CacheConfiguration {
    pub use_moves_cache: bool,
    pub use_vision_cache: bool,
    pub use_eval_cache: bool,
}

pub fn configure_global_cache(config: CacheConfiguration) {
    GLOBAL_CACHE_CONFIG
        .set(config)
        .expect("unable to configure cache");
}

impl CacheConfiguration {
    pub fn get() -> &'static CacheConfiguration {
        GLOBAL_CACHE_CONFIG.get_or_init(|| CacheConfiguration::default())
    }

    pub fn disabled() -> Self {
        CacheConfiguration {
            use_moves_cache: false,
            use_vision_cache: false,
            use_eval_cache: false,
        }
    }
}

impl Default for CacheConfiguration {
    fn default() -> Self {
        CacheConfiguration {
            use_moves_cache: true,
            use_vision_cache: true,
            use_eval_cache: true,
        }
    }
}
