// Generated macro for impl_400 (impl)
macro_rules! Depcrate_collections_hash_mapimpl_400 {
() => {
// Module: crate::collections::hash::map
// Provides: {"impl_400"}
// Dependencies: {}
# [stable (feature = "std_debug" , since = "1.16.0")] impl < K : Debug , V : Debug > fmt :: Debug for IntoIter < K , V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . iter ()) . finish () } }
};
}
