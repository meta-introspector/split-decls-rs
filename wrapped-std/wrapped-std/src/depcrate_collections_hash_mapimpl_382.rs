// Generated macro for impl_382 (impl)
macro_rules! Depcrate_collections_hash_mapimpl_382 {
() => {
// Module: crate::collections::hash::map
// Provides: {"impl_382"}
// Dependencies: {}
# [stable (feature = "debug_hash_map" , since = "1.12.0")] impl < K : Debug , V > Debug for VacantEntry < '_ , K , V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("VacantEntry") . field (self . key ()) . finish () } }
};
}
