// Generated macro for impl_454 (impl)
macro_rules! Depcrate_collections_hash_setimpl_454 {
() => {
// Module: crate::collections::hash::set
// Provides: {"impl_454"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T , S > fmt :: Debug for HashSet < T , S > where T : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_set () . entries (self . iter ()) . finish () } }
};
}
