// Generated macro for impl_36 (impl)
macro_rules! Depcrate_hash_cacheimpl_36 {
() => {
// Module: crate::hash_cache
// Provides: {"impl_36"}
// Dependencies: {}
impl < K , V > HashCache < K , V , RandomState > { # [doc = " Creates an empty default [`HashCache`]."] # [doc = ""] # [doc = " The maximum capacity is set to [`DEFAULT_MAXIMUM_CAPACITY`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use scc::HashCache;"] # [doc = ""] # [doc = " let hashcache: HashCache<u64, u32> = HashCache::new();"] # [doc = ""] # [doc = " let result = hashcache.capacity();"] # [doc = " assert_eq!(result, 0);"] # [doc = " ```"] # [inline] # [must_use] pub fn new () -> Self { Self :: default () } # [doc = " Creates an empty [`HashCache`] with the specified capacity."] # [doc = ""] # [doc = " The actual capacity is equal to or greater than `minimum_capacity` unless it is greater than"] # [doc = " `1 << (usize::BITS - 1)`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use scc::HashCache;"] # [doc = ""] # [doc = " let hashcache: HashCache<u64, u32> = HashCache::with_capacity(1000, 2000);"] # [doc = ""] # [doc = " let result = hashcache.capacity();"] # [doc = " assert_eq!(result, 1024);"] # [doc = ""] # [doc = " let hashcache: HashCache<u64, u32> = HashCache::with_capacity(0, 0);"] # [doc = " let result = hashcache.capacity_range();"] # [doc = " assert_eq!(result, 0..=64);"] # [doc = " ```"] # [inline] # [must_use] pub fn with_capacity (minimum_capacity : usize , maximum_capacity : usize) -> Self { Self :: with_capacity_and_hasher (minimum_capacity , maximum_capacity , RandomState :: new ()) } }
};
}
