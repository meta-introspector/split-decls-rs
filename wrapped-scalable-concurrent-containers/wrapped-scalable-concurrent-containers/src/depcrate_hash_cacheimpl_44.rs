// Generated macro for impl_44 (impl)
macro_rules! Depcrate_hash_cacheimpl_44 {
() => {
// Module: crate::hash_cache
// Provides: {"impl_44"}
// Dependencies: {}
impl < 'h , K , V , H > Entry < 'h , K , V , H > where K : Eq + Hash , V : Default , H : BuildHasher , { # [doc = " Ensures a value is in the entry by putting the default value if empty."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use scc::HashCache;"] # [doc = ""] # [doc = " let hashcache: HashCache<u64, u32> = HashCache::default();"] # [doc = " hashcache.entry_sync(11).or_default();"] # [doc = " assert_eq!(*hashcache.get_sync(&11).unwrap().get(), 0);"] # [doc = " ```"] # [inline] pub fn or_default (self) -> (EvictedEntry < K , V > , OccupiedEntry < 'h , K , V , H >) { match self { Self :: Occupied (o) => (None , o) , Self :: Vacant (v) => v . put_entry (Default :: default ()) , } } }
};
}
