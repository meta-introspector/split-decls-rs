// Generated macro for IterMut (struct)
macro_rules! Depcrate_lru_disk_cache_lru_cacheIterMut {
() => {
// Module: crate::lru_disk_cache::lru_cache
// Provides: {"IterMut"}
// Dependencies: {}
# [doc = " An iterator over a cache's key-value pairs in least- to most-recently-used order with mutable"] # [doc = " references to the values."] # [doc = ""] # [doc = " Accessing a cache through the iterator does _not_ affect the cache's LRU state."] pub struct IterMut < 'a , K , V > (linked_hash_map :: IterMut < 'a , K , V >) ;
};
}
