// Generated macro for impl_418 (impl)
macro_rules! Depcrate_collections_hash_mapimpl_418 {
() => {
// Module: crate::collections::hash::map
// Provides: {"impl_418"}
// Dependencies: {}
# [stable (feature = "map_into_keys_values" , since = "1.54.0")] impl < K , V : Debug > fmt :: Debug for IntoValues < K , V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . inner . iter () . map (| (_ , v) | v)) . finish () } }
};
}
