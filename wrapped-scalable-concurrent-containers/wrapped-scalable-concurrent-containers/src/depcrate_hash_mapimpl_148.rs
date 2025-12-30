// Generated macro for impl_148 (impl)
macro_rules! Depcrate_hash_mapimpl_148 {
() => {
// Module: crate::hash_map
// Provides: {"impl_148"}
// Dependencies: {}
impl < K , V , H > DerefMut for OccupiedEntry < '_ , K , V , H > where K : Eq + Hash , H : BuildHasher , { # [inline] fn deref_mut (& mut self) -> & mut Self :: Target { self . get_mut () } }
};
}
