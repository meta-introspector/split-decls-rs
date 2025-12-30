// Generated macro for impl_102 (impl)
macro_rules! Depcrate_hash_indeximpl_102 {
() => {
// Module: crate::hash_index
// Provides: {"impl_102"}
// Dependencies: {}
impl < K , V , H > Deref for Reserve < '_ , K , V , H > where K : Eq + Hash , H : BuildHasher , { type Target = HashIndex < K , V , H > ; # [inline] fn deref (& self) -> & Self :: Target { self . hashindex } }
};
}
