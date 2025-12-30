// Generated macro for Difference (struct)
macro_rules! Depcrate_collections_hash_setDifference {
() => {
// Module: crate::collections::hash::set
// Provides: {"Difference"}
// Dependencies: {}
# [doc = " A lazy iterator producing elements in the difference of `HashSet`s."] # [doc = ""] # [doc = " This `struct` is created by the [`difference`] method on [`HashSet`]."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " [`difference`]: HashSet::difference"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::collections::HashSet;"] # [doc = ""] # [doc = " let a = HashSet::from([1, 2, 3]);"] # [doc = " let b = HashSet::from([4, 2, 3, 4]);"] # [doc = ""] # [doc = " let mut difference = a.difference(&b);"] # [doc = " ```"] # [must_use = "this returns the difference as an iterator, \
              without modifying either input set"] # [stable (feature = "rust1" , since = "1.0.0")] pub struct Difference < 'a , T : 'a , S : 'a > { iter : Iter < 'a , T > , other : & 'a HashSet < T , S > , }
};
}
