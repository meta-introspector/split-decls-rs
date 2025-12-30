// Generated macro for impl_155 (impl)
macro_rules! Depcrate_hash_mapimpl_155 {
() => {
// Module: crate::hash_map
// Provides: {"impl_155"}
// Dependencies: {}
impl < K , V , H > AsRef < HashMap < K , V , H > > for Reserve < '_ , K , V , H > where K : Eq + Hash , H : BuildHasher , { # [inline] fn as_ref (& self) -> & HashMap < K , V , H > { self . hashmap } }
};
}
