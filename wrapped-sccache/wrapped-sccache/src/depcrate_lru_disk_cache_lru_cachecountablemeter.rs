// Generated macro for CountableMeter (trait)
macro_rules! Depcrate_lru_disk_cache_lru_cacheCountableMeter {
() => {
// Module: crate::lru_disk_cache::lru_cache
// Provides: {"CountableMeter"}
// Dependencies: {}
# [doc = " A trait to allow the default `Count` measurement to not store an"] # [doc = " extraneous counter."] pub trait CountableMeter < K , V > : Meter < K , V > { # [doc = " Add `amount` to `current` and return the sum."] fn add (& self , current : Self :: Measure , amount : Self :: Measure) -> Self :: Measure ; # [doc = " Subtract `amount` from `current` and return the difference."] fn sub (& self , current : Self :: Measure , amount : Self :: Measure) -> Self :: Measure ; # [doc = " Return `current` as a `usize` if possible, otherwise return `None`."] # [doc = ""] # [doc = " If this method returns `None` the cache will use the number of cache entries as"] # [doc = " its size."] fn size (& self , current : Self :: Measure) -> Option < u64 > ; }
};
}
