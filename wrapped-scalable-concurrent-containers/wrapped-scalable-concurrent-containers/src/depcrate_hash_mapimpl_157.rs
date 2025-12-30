// Generated macro for impl_157 (impl)
macro_rules! Depcrate_hash_mapimpl_157 {
() => {
// Module: crate::hash_map
// Provides: {"impl_157"}
// Dependencies: {}
impl < K , V , H > Deref for Reserve < '_ , K , V , H > where K : Eq + Hash , H : BuildHasher , { type Target = HashMap < K , V , H > ; # [inline] fn deref (& self) -> & Self :: Target { self . hashmap } }
};
}
