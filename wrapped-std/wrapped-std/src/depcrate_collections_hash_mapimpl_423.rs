// Generated macro for impl_423 (impl)
macro_rules! Depcrate_collections_hash_mapimpl_423 {
() => {
// Module: crate::collections::hash::map
// Provides: {"impl_423"}
// Dependencies: {}
# [stable (feature = "hash_extract_if" , since = "1.88.0")] impl < K , V , F > Iterator for ExtractIf < '_ , K , V , F > where F : FnMut (& K , & mut V) -> bool , { type Item = (K , V) ; # [inline] fn next (& mut self) -> Option < (K , V) > { self . base . next () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . base . size_hint () } }
};
}
