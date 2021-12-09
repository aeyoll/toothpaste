use futures::lock::Mutex;

use std::num::NonZeroUsize;
use std::sync::Arc;

use clru::{CLruCache, CLruCacheConfig, WeightScale};
use fnv::FnvBuildHasher;

pub type HtmlCache = Arc<Mutex<CLruCache<String, String, FnvBuildHasher, StringScale>>>;
pub struct StringScale;

impl WeightScale<String, String> for StringScale {
    fn weight(&self, _key: &String, value: &String) -> usize {
        value.len() + std::mem::size_of::<String>()
    }
}

// Initializes the cache
pub fn create_cache() -> HtmlCache {
    let capacity = NonZeroUsize::new(67108864).unwrap(); // 64 MB
    let config = CLruCacheConfig::new(capacity)
        .with_hasher(FnvBuildHasher::default())
        .with_scale(StringScale);

    let cache = CLruCache::with_config(config);
    Arc::new(Mutex::new(cache))
}
