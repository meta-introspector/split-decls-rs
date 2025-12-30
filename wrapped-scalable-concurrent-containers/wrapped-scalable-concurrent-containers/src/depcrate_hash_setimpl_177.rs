// Generated macro for impl_177 (impl)
macro_rules! Depcrate_hash_setimpl_177 {
() => {
// Module: crate::hash_set
// Provides: {"impl_177"}
// Dependencies: {}
impl < K , H > Clone for HashSet < K , H > where K : Clone + Eq + Hash , H : BuildHasher + Clone , { # [inline] fn clone (& self) -> Self { Self { map : self . map . clone () , } } }
};
}
