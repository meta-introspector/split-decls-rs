// Generated macro for impl_1212 (impl)
macro_rules! Depcrate_lru_disk_cache_lru_cacheimpl_1212 {
() => {
// Module: crate::lru_disk_cache::lru_cache
// Provides: {"impl_1212"}
// Dependencies: {}
impl < K : Eq + Hash , V > LruCache < K , V > { # [doc = " Creates an empty cache that can hold at most `capacity` items."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " use lru_cache::LruCache;"] # [doc = " let mut cache: LruCache<i32, &str> = LruCache::new(10);"] # [doc = " ```"] pub fn new (capacity : u64) -> Self { LruCache { map : LinkedHashMap :: new () , current_measure : () , max_capacity : capacity , meter : Count , } } }
};
}
