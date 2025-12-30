// Generated macro for Iter (struct)
macro_rules! Depcrate_lru_disk_cache_lru_cacheIter {
() => {
// Module: crate::lru_disk_cache::lru_cache
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " An iterator over a cache's key-value pairs in least- to most-recently-used order."] # [doc = ""] # [doc = " Accessing a cache through the iterator does _not_ affect the cache's LRU state."] pub struct Iter < 'a , K , V > (linked_hash_map :: Iter < 'a , K , V >) ;
};
}
