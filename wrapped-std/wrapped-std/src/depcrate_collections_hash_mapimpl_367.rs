// Generated macro for impl_367 (impl)
macro_rules! Depcrate_collections_hash_mapimpl_367 {
() => {
// Module: crate::collections::hash::map
// Provides: {"impl_367"}
// Dependencies: {}
# [stable (feature = "std_debug" , since = "1.16.0")] impl < K , V : Debug > fmt :: Debug for Values < '_ , K , V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . clone ()) . finish () } }
};
}
