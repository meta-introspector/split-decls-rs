// Generated macro for SymmetricDifference (struct)
macro_rules! Depcrate_collections_hash_setSymmetricDifference {
() => {
// Module: crate::collections::hash::set
// Provides: {"SymmetricDifference"}
// Dependencies: {}
# [doc = " A lazy iterator producing elements in the symmetric difference of `HashSet`s."] # [doc = ""] # [doc = " This `struct` is created by the [`symmetric_difference`] method on"] # [doc = " [`HashSet`]. See its documentation for more."] # [doc = ""] # [doc = " [`symmetric_difference`]: HashSet::symmetric_difference"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::collections::HashSet;"] # [doc = ""] # [doc = " let a = HashSet::from([1, 2, 3]);"] # [doc = " let b = HashSet::from([4, 2, 3, 4]);"] # [doc = ""] # [doc = " let mut intersection = a.symmetric_difference(&b);"] # [doc = " ```"] # [must_use = "this returns the difference as an iterator, \
              without modifying either input set"] # [stable (feature = "rust1" , since = "1.0.0")] pub struct SymmetricDifference < 'a , T : 'a , S : 'a > { iter : Chain < Difference < 'a , T , S > , Difference < 'a , T , S > > , }
};
}
