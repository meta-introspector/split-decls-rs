// Generated macro for impl_96 (impl)
macro_rules! Depcrate_hash_indeximpl_96 {
() => {
// Module: crate::hash_index
// Provides: {"impl_96"}
// Dependencies: {}
impl < K , V , H > Deref for OccupiedEntry < '_ , K , V , H > where K : Debug + Eq + Hash , V : Debug , H : BuildHasher , { type Target = V ; # [inline] fn deref (& self) -> & Self :: Target { self . get () } }
};
}
