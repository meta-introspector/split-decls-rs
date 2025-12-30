// Generated macro for impl_49 (impl)
macro_rules! Depcrate_hash_cacheimpl_49 {
() => {
// Module: crate::hash_cache
// Provides: {"impl_49"}
// Dependencies: {}
impl < K , V , H > DerefMut for OccupiedEntry < '_ , K , V , H > where K : Eq + Hash , H : BuildHasher , { # [inline] fn deref_mut (& mut self) -> & mut Self :: Target { self . get_mut () } }
};
}
