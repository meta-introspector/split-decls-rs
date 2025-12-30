// Generated macro for IntoIter (struct)
macro_rules! Depcrate_lru_disk_cache_lru_cacheIntoIter {
() => {
// Module: crate::lru_disk_cache::lru_cache
// Provides: {"IntoIter"}
// Dependencies: {}
# [doc = " An iterator over a cache's key-value pairs in least- to most-recently-used order."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " use lru_cache::LruCache;"] # [doc = ""] # [doc = " let mut cache = LruCache::new(2);"] # [doc = ""] # [doc = " cache.insert(1, 10);"] # [doc = " cache.insert(2, 20);"] # [doc = " cache.insert(3, 30);"] # [doc = ""] # [doc = " let mut n = 2;"] # [doc = ""] # [doc = " for (k, v) in cache {"] # [doc = "     assert_eq!(k, n);"] # [doc = "     assert_eq!(v, n * 10);"] # [doc = "     n += 1;"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(n, 4);"] # [doc = " ```"] # [derive (Clone)] pub struct IntoIter < K , V > (linked_hash_map :: IntoIter < K , V >) ;
};
}
