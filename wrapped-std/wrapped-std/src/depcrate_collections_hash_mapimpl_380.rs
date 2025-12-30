// Generated macro for impl_380 (impl)
macro_rules! Depcrate_collections_hash_mapimpl_380 {
() => {
// Module: crate::collections::hash::map
// Provides: {"impl_380"}
// Dependencies: {}
# [stable (feature = "debug_hash_map" , since = "1.12.0")] impl < K : Debug , V : Debug > Debug for OccupiedEntry < '_ , K , V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("OccupiedEntry") . field ("key" , self . key ()) . field ("value" , self . get ()) . finish_non_exhaustive () } }
};
}
