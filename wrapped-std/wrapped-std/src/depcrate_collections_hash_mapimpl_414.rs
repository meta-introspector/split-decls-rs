// Generated macro for impl_414 (impl)
macro_rules! Depcrate_collections_hash_mapimpl_414 {
() => {
// Module: crate::collections::hash::map
// Provides: {"impl_414"}
// Dependencies: {}
# [stable (feature = "map_into_keys_values" , since = "1.54.0")] impl < K : Debug , V > fmt :: Debug for IntoKeys < K , V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . inner . iter () . map (| (k , _) | k)) . finish () } }
};
}
