// Generated macro for impl_100 (impl)
macro_rules! Depcrate_hash_indeximpl_100 {
() => {
// Module: crate::hash_index
// Provides: {"impl_100"}
// Dependencies: {}
impl < K , V , H > AsRef < HashIndex < K , V , H > > for Reserve < '_ , K , V , H > where K : Eq + Hash , H : BuildHasher , { # [inline] fn as_ref (& self) -> & HashIndex < K , V , H > { self . hashindex } }
};
}
