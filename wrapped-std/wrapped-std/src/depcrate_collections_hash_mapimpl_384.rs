// Generated macro for impl_384 (impl)
macro_rules! Depcrate_collections_hash_mapimpl_384 {
() => {
// Module: crate::collections::hash::map
// Provides: {"impl_384"}
// Dependencies: {}
# [unstable (feature = "map_try_insert" , issue = "82766")] impl < K : Debug , V : Debug > Debug for OccupiedError < '_ , K , V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("OccupiedError") . field ("key" , self . entry . key ()) . field ("old_value" , self . entry . get ()) . field ("new_value" , & self . value) . finish_non_exhaustive () } }
};
}
