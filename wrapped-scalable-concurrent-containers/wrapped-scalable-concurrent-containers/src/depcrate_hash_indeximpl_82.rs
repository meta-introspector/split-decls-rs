// Generated macro for impl_82 (impl)
macro_rules! Depcrate_hash_indeximpl_82 {
() => {
// Module: crate::hash_index
// Provides: {"impl_82"}
// Dependencies: {}
impl < K , V , H > Clone for HashIndex < K , V , H > where K : Clone + Eq + Hash , V : Clone , H : BuildHasher + Clone , { # [inline] fn clone (& self) -> Self { let self_clone = Self :: with_capacity_and_hasher (self . capacity () , self . hasher () . clone ()) ; for (k , v) in self . iter (& Guard :: new ()) { let _result = self_clone . insert_sync (k . clone () , v . clone ()) ; } self_clone } }
};
}
