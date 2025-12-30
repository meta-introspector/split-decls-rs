// Generated macro for impl_54 (impl)
macro_rules! Depcrate_hash_cacheimpl_54 {
() => {
// Module: crate::hash_cache
// Provides: {"impl_54"}
// Dependencies: {}
impl < K , V > DerefMut for ConsumableEntry < '_ , '_ , K , V > { # [inline] fn deref_mut (& mut self) -> & mut Self :: Target { self . locked_bucket . entry_mut (self . entry_ptr) } }
};
}
