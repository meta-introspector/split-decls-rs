// Generated macro for impl_153 (impl)
macro_rules! Depcrate_hash_mapimpl_153 {
() => {
// Module: crate::hash_map
// Provides: {"impl_153"}
// Dependencies: {}
impl < K , V > DerefMut for ConsumableEntry < '_ , '_ , K , V > { # [inline] fn deref_mut (& mut self) -> & mut Self :: Target { self . locked_bucket . entry_mut (self . entry_ptr) } }
};
}
