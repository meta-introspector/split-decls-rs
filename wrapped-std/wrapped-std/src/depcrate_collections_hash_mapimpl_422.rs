// Generated macro for impl_422 (impl)
macro_rules! Depcrate_collections_hash_mapimpl_422 {
() => {
// Module: crate::collections::hash::map
// Provides: {"impl_422"}
// Dependencies: {}
# [stable (feature = "std_debug" , since = "1.16.0")] impl < K , V > fmt :: Debug for Drain < '_ , K , V > where K : fmt :: Debug , V : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . iter ()) . finish () } }
};
}
