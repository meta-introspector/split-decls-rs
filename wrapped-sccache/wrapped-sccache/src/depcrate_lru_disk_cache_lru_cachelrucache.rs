// Generated macro for LruCache (struct)
macro_rules! Depcrate_lru_disk_cache_lru_cacheLruCache {
() => {
// Module: crate::lru_disk_cache::lru_cache
// Provides: {"LruCache"}
// Dependencies: {}
# [doc = " An LRU cache."] # [derive (Clone)] pub struct LruCache < K : Eq + Hash , V , S : BuildHasher = RandomState , M : CountableMeter < K , V > = Count > { map : LinkedHashMap < K , V , S > , current_measure : M :: Measure , max_capacity : u64 , meter : M , }
};
}
