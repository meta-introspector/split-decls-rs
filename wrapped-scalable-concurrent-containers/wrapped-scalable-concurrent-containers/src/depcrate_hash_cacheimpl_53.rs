// Generated macro for impl_53 (impl)
macro_rules! Depcrate_hash_cacheimpl_53 {
() => {
// Module: crate::hash_cache
// Provides: {"impl_53"}
// Dependencies: {}
impl < K , V > Deref for ConsumableEntry < '_ , '_ , K , V > { type Target = (K , V) ; # [inline] fn deref (& self) -> & Self :: Target { self . locked_bucket . entry (self . entry_ptr) } }
};
}
