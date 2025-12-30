// Generated macro for impl_343 (impl)
macro_rules! Depcrate_collections_hash_mapimpl_343 {
() => {
// Module: crate::collections::hash::map
// Provides: {"impl_343"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < K , V , S > Clone for HashMap < K , V , S > where K : Clone , V : Clone , S : Clone , { # [inline] fn clone (& self) -> Self { Self { base : self . base . clone () } } # [inline] fn clone_from (& mut self , source : & Self) { self . base . clone_from (& source . base) ; } }
};
}
