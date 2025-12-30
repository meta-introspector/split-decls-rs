// Generated macro for impl_1213 (impl)
macro_rules! Depcrate_lru_disk_cache_lru_cacheimpl_1213 {
() => {
// Module: crate::lru_disk_cache::lru_cache
// Provides: {"impl_1213"}
// Dependencies: {}
impl < K : Eq + Hash , V , M : CountableMeter < K , V > > LruCache < K , V , RandomState , M > { # [doc = " Creates an empty cache that can hold at most `capacity` as measured by `meter`."] # [doc = ""] # [doc = " You can implement the [`Meter`][meter] trait to allow custom metrics."] # [doc = ""] # [doc = " [meter]: trait.Meter.html"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " use lru_cache::{LruCache, Meter};"] # [doc = " use std::borrow::Borrow;"] # [doc = ""] # [doc = " /// Measure Vec items by their length"] # [doc = " struct VecLen;"] # [doc = ""] # [doc = " impl<K, T> Meter<K, Vec<T>> for VecLen {"] # [doc = "     // Use `Measure = usize` or implement `CountableMeter` as well."] # [doc = "     type Measure = usize;"] # [doc = "     fn measure<Q: ?Sized>(&self, _: &Q, v: &Vec<T>) -> usize"] # [doc = "         where K: Borrow<Q>"] # [doc = "     {"] # [doc = "         v.len()"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " let mut cache = LruCache::with_meter(5, VecLen);"] # [doc = " cache.insert(1, vec![1, 2]);"] # [doc = " assert_eq!(cache.size(), 2);"] # [doc = " cache.insert(2, vec![3, 4]);"] # [doc = " cache.insert(3, vec![5, 6]);"] # [doc = " assert_eq!(cache.size(), 4);"] # [doc = " assert_eq!(cache.len(), 2);"] # [doc = " ```"] pub fn with_meter (capacity : u64 , meter : M) -> LruCache < K , V , RandomState , M > { LruCache { map : LinkedHashMap :: new () , current_measure : Default :: default () , max_capacity : capacity , meter , } } }
};
}
