// Generated macro for impl_152 (impl)
macro_rules! Depcrate_hash_mapimpl_152 {
() => {
// Module: crate::hash_map
// Provides: {"impl_152"}
// Dependencies: {}
impl < K , V > Deref for ConsumableEntry < '_ , '_ , K , V > { type Target = (K , V) ; # [inline] fn deref (& self) -> & Self :: Target { self . locked_bucket . entry (self . entry_ptr) } }
};
}
