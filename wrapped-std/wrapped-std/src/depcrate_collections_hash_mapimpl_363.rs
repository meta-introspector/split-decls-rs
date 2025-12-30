// Generated macro for impl_363 (impl)
macro_rules! Depcrate_collections_hash_mapimpl_363 {
() => {
// Module: crate::collections::hash::map
// Provides: {"impl_363"}
// Dependencies: {}
# [stable (feature = "std_debug" , since = "1.16.0")] impl < K : Debug , V > fmt :: Debug for Keys < '_ , K , V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . clone ()) . finish () } }
};
}
