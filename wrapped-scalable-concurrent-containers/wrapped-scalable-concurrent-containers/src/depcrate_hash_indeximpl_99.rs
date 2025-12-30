// Generated macro for impl_99 (impl)
macro_rules! Depcrate_hash_indeximpl_99 {
() => {
// Module: crate::hash_index
// Provides: {"impl_99"}
// Dependencies: {}
impl < K , V , H > Reserve < '_ , K , V , H > where K : Eq + Hash , H : BuildHasher , { # [doc = " Returns the number of reserved slots."] # [inline] # [must_use] pub fn additional_capacity (& self) -> usize { self . additional } }
};
}
