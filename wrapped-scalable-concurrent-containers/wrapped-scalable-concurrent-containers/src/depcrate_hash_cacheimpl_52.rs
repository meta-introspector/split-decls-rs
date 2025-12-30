// Generated macro for impl_52 (impl)
macro_rules! Depcrate_hash_cacheimpl_52 {
() => {
// Module: crate::hash_cache
// Provides: {"impl_52"}
// Dependencies: {}
impl < K , V > ConsumableEntry < '_ , '_ , K , V > { # [doc = " Consumes the entry by moving out the key and value."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use scc::HashCache;"] # [doc = ""] # [doc = " let hashcache: HashCache<u64, u32> = HashCache::default();"] # [doc = ""] # [doc = " assert!(hashcache.put_sync(1, 0).is_ok());"] # [doc = " assert!(hashcache.put_sync(2, 1).is_ok());"] # [doc = " assert!(hashcache.put_sync(3, 2).is_ok());"] # [doc = ""] # [doc = " let mut consumed = None;"] # [doc = ""] # [doc = " hashcache.iter_mut_sync(|entry| {"] # [doc = "     if entry.0 == 1 {"] # [doc = "         consumed.replace(entry.consume().1);"] # [doc = "     }"] # [doc = "     true"] # [doc = " });"] # [doc = ""] # [doc = " assert!(!hashcache.contains_sync(&1));"] # [doc = " assert_eq!(consumed, Some(0));"] # [doc = " ```"] # [inline] # [must_use] pub fn consume (self) -> (K , V) { * self . remove_probe |= true ; self . locked_bucket . writer . remove (self . locked_bucket . data_block , self . entry_ptr , self . guard) } }
};
}
