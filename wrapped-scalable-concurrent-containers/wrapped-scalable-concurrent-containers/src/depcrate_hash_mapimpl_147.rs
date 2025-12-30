// Generated macro for impl_147 (impl)
macro_rules! Depcrate_hash_mapimpl_147 {
() => {
// Module: crate::hash_map
// Provides: {"impl_147"}
// Dependencies: {}
impl < K , V , H > Deref for OccupiedEntry < '_ , K , V , H > where K : Eq + Hash , H : BuildHasher , { type Target = V ; # [inline] fn deref (& self) -> & Self :: Target { self . get () } }
};
}
