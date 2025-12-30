// Generated macro for impl_387 (impl)
macro_rules! Depcrate_collections_hash_mapimpl_387 {
() => {
// Module: crate::collections::hash::map
// Provides: {"impl_387"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < 'a , K , V , S > IntoIterator for & 'a HashMap < K , V , S > { type Item = (& 'a K , & 'a V) ; type IntoIter = Iter < 'a , K , V > ; # [inline] # [rustc_lint_query_instability] fn into_iter (self) -> Iter < 'a , K , V > { self . iter () } }
};
}
