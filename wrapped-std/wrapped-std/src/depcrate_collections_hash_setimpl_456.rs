// Generated macro for impl_456 (impl)
macro_rules! Depcrate_collections_hash_setimpl_456 {
() => {
// Module: crate::collections::hash::set
// Provides: {"impl_456"}
// Dependencies: {}
# [stable (feature = "std_collections_from_array" , since = "1.56.0")] impl < T , const N : usize > From < [T ; N] > for HashSet < T , RandomState > where T : Eq + Hash , { # [doc = " Converts a `[T; N]` into a `HashSet<T>`."] # [doc = ""] # [doc = " If the array contains any equal values,"] # [doc = " all but one will be dropped."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::collections::HashSet;"] # [doc = ""] # [doc = " let set1 = HashSet::from([1, 2, 3, 4]);"] # [doc = " let set2: HashSet<_> = [1, 2, 3, 4].into();"] # [doc = " assert_eq!(set1, set2);"] # [doc = " ```"] fn from (arr : [T ; N]) -> Self { Self :: from_iter (arr) } }
};
}
