// Generated macro for CountableMeterWithMeasure (trait)
macro_rules! Depcrate_lru_disk_cache_lru_cacheCountableMeterWithMeasure {
() => {
// Module: crate::lru_disk_cache::lru_cache
// Provides: {"CountableMeterWithMeasure"}
// Dependencies: {}
pub trait CountableMeterWithMeasure < K , V , M > { # [doc = " Add `amount` to `current` and return the sum."] fn meter_add (& self , current : M , amount : M) -> M ; # [doc = " Subtract `amount` from `current` and return the difference."] fn meter_sub (& self , current : M , amount : M) -> M ; # [doc = " Return `current` as a `usize` if possible, otherwise return `None`."] # [doc = ""] # [doc = " If this method returns `None` the cache will use the number of cache entries as"] # [doc = " its size."] fn meter_size (& self , current : M) -> Option < u64 > ; }
};
}
