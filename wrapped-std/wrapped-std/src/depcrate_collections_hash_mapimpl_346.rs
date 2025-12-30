// Generated macro for impl_346 (impl)
macro_rules! Depcrate_collections_hash_mapimpl_346 {
() => {
// Module: crate::collections::hash::map
// Provides: {"impl_346"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < K , V , S > Debug for HashMap < K , V , S > where K : Debug , V : Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_map () . entries (self . iter ()) . finish () } }
};
}
