// Generated macro for impl_48 (impl)
macro_rules! Depcrate_hash_cacheimpl_48 {
() => {
// Module: crate::hash_cache
// Provides: {"impl_48"}
// Dependencies: {}
impl < K , V , H > Deref for OccupiedEntry < '_ , K , V , H > where K : Eq + Hash , H : BuildHasher , { type Target = V ; # [inline] fn deref (& self) -> & Self :: Target { self . get () } }
};
}
