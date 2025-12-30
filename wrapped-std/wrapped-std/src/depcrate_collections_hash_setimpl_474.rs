// Generated macro for impl_474 (impl)
macro_rules! Depcrate_collections_hash_setimpl_474 {
() => {
// Module: crate::collections::hash::set
// Provides: {"impl_474"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < 'a , T , S > IntoIterator for & 'a HashSet < T , S > { type Item = & 'a T ; type IntoIter = Iter < 'a , T > ; # [inline] # [rustc_lint_query_instability] fn into_iter (self) -> Iter < 'a , T > { self . iter () } }
};
}
