// Generated macro for impl_135 (impl)
macro_rules! Depcrate_hash_mapimpl_135 {
() => {
// Module: crate::hash_map
// Provides: {"impl_135"}
// Dependencies: {}
impl < K , V , H > Clone for HashMap < K , V , H > where K : Clone + Eq + Hash , V : Clone , H : BuildHasher + Clone , { # [inline] fn clone (& self) -> Self { let self_clone = Self :: with_capacity_and_hasher (self . capacity () , self . hasher () . clone ()) ; self . iter_sync (| k , v | { let _result = self_clone . insert_sync (k . clone () , v . clone ()) ; true }) ; self_clone } }
};
}
