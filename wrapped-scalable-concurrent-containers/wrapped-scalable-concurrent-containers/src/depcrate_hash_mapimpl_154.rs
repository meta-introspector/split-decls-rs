// Generated macro for impl_154 (impl)
macro_rules! Depcrate_hash_mapimpl_154 {
() => {
// Module: crate::hash_map
// Provides: {"impl_154"}
// Dependencies: {}
impl < K , V , H > Reserve < '_ , K , V , H > where K : Eq + Hash , H : BuildHasher , { # [doc = " Returns the number of reserved slots."] # [inline] # [must_use] pub fn additional_capacity (& self) -> usize { self . additional } }
};
}
