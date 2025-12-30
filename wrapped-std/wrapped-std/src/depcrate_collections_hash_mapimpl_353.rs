// Generated macro for impl_353 (impl)
macro_rules! Depcrate_collections_hash_mapimpl_353 {
() => {
// Module: crate::collections::hash::map
// Provides: {"impl_353"}
// Dependencies: {}
# [stable (feature = "std_debug" , since = "1.16.0")] impl < K : Debug , V : Debug > fmt :: Debug for Iter < '_ , K , V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . clone ()) . finish () } }
};
}
