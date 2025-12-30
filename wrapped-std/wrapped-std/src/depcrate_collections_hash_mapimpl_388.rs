// Generated macro for impl_388 (impl)
macro_rules! Depcrate_collections_hash_mapimpl_388 {
() => {
// Module: crate::collections::hash::map
// Provides: {"impl_388"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < 'a , K , V , S > IntoIterator for & 'a mut HashMap < K , V , S > { type Item = (& 'a K , & 'a mut V) ; type IntoIter = IterMut < 'a , K , V > ; # [inline] # [rustc_lint_query_instability] fn into_iter (self) -> IterMut < 'a , K , V > { self . iter_mut () } }
};
}
