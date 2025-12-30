// Generated macro for impl_410 (impl)
macro_rules! Depcrate_collections_hash_mapimpl_410 {
() => {
// Module: crate::collections::hash::map
// Provides: {"impl_410"}
// Dependencies: {}
# [stable (feature = "std_debug" , since = "1.16.0")] impl < K , V : fmt :: Debug > fmt :: Debug for ValuesMut < '_ , K , V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . inner . iter () . map (| (_ , val) | val)) . finish () } }
};
}
