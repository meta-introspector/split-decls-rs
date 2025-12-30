// Generated macro for impl_489 (impl)
macro_rules! Depcrate_collections_hash_setimpl_489 {
() => {
// Module: crate::collections::hash::set
// Provides: {"impl_489"}
// Dependencies: {}
# [stable (feature = "hash_extract_if" , since = "1.88.0")] impl < K , F > Iterator for ExtractIf < '_ , K , F > where F : FnMut (& K) -> bool , { type Item = K ; # [inline] fn next (& mut self) -> Option < K > { self . base . next () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . base . size_hint () } }
};
}
