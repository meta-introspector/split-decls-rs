// Generated macro for Union (struct)
macro_rules! Depcrate_collections_hash_setUnion {
() => {
// Module: crate::collections::hash::set
// Provides: {"Union"}
// Dependencies: {}
# [doc = " A lazy iterator producing elements in the union of `HashSet`s."] # [doc = ""] # [doc = " This `struct` is created by the [`union`] method on [`HashSet`]."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " [`union`]: HashSet::union"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::collections::HashSet;"] # [doc = ""] # [doc = " let a = HashSet::from([1, 2, 3]);"] # [doc = " let b = HashSet::from([4, 2, 3, 4]);"] # [doc = ""] # [doc = " let mut union_iter = a.union(&b);"] # [doc = " ```"] # [must_use = "this returns the union as an iterator, \
              without modifying either input set"] # [stable (feature = "rust1" , since = "1.0.0")] pub struct Union < 'a , T : 'a , S : 'a > { iter : Chain < Iter < 'a , T > , Difference < 'a , T , S > > , }
};
}
