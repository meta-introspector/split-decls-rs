// Generated macro for impl_480 (impl)
macro_rules! Depcrate_collections_hash_setimpl_480 {
() => {
// Module: crate::collections::hash::set
// Provides: {"impl_480"}
// Dependencies: {}
# [stable (feature = "std_debug" , since = "1.16.0")] impl < K : fmt :: Debug > fmt :: Debug for Iter < '_ , K > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . clone ()) . finish () } }
};
}
