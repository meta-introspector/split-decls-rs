// Generated macro for Meter (trait)
macro_rules! Depcrate_lru_disk_cache_lru_cacheMeter {
() => {
// Module: crate::lru_disk_cache::lru_cache
// Provides: {"Meter"}
// Dependencies: {}
# [doc = " A trait for measuring the size of a cache entry."] # [doc = ""] # [doc = " If you implement this trait, you should use `usize` as the `Measure` type, otherwise you will"] # [doc = " also have to implement [`CountableMeter`][countablemeter]."] # [doc = ""] # [doc = " [countablemeter]: trait.Meter.html"] pub trait Meter < K , V > { # [doc = " The type used to store measurements."] type Measure : Default + Copy ; # [doc = " Calculate the size of `key` and `value`."] fn measure < Q : ? Sized > (& self , key : & Q , value : & V) -> Self :: Measure where K : Borrow < Q > ; }
};
}
