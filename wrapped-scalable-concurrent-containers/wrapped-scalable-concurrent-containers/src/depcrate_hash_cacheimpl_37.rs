// Generated macro for impl_37 (impl)
macro_rules! Depcrate_hash_cacheimpl_37 {
() => {
// Module: crate::hash_cache
// Provides: {"impl_37"}
// Dependencies: {}
impl < K , V , H > Default for HashCache < K , V , H > where H : BuildHasher + Default , { # [doc = " Creates an empty default [`HashCache`]."] # [doc = ""] # [doc = " The maximum capacity is set to [`DEFAULT_MAXIMUM_CAPACITY`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use scc::HashCache;"] # [doc = ""] # [doc = " let hashcache: HashCache<u64, u32> = HashCache::default();"] # [doc = ""] # [doc = " let result = hashcache.capacity();"] # [doc = " assert_eq!(result, 0);"] # [doc = " ```"] # [inline] fn default () -> Self { Self :: with_hasher (H :: default ()) } }
};
}
