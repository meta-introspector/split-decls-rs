// Generated macro for impl_1209 (impl)
macro_rules! Depcrate_lru_disk_cache_lru_cacheimpl_1209 {
() => {
// Module: crate::lru_disk_cache::lru_cache
// Provides: {"impl_1209"}
// Dependencies: {}
# [doc = " For any other `Meter` with `Measure=usize`, just do the simple math."] impl < K , V , T > CountableMeterWithMeasure < K , V , usize > for T where T : Meter < K , V > , { fn meter_add (& self , current : usize , amount : usize) -> usize { current + amount } fn meter_sub (& self , current : usize , amount : usize) -> usize { current - amount } fn meter_size (& self , current : usize) -> Option < u64 > { Some (current as u64) } }
};
}
