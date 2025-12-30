// Generated macro for Intersection (struct)
macro_rules! Depcrate_collections_hash_setIntersection {
() => {
// Module: crate::collections::hash::set
// Provides: {"Intersection"}
// Dependencies: {}
# [doc = " A lazy iterator producing elements in the intersection of `HashSet`s."] # [doc = ""] # [doc = " This `struct` is created by the [`intersection`] method on [`HashSet`]."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " [`intersection`]: HashSet::intersection"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::collections::HashSet;"] # [doc = ""] # [doc = " let a = HashSet::from([1, 2, 3]);"] # [doc = " let b = HashSet::from([4, 2, 3, 4]);"] # [doc = ""] # [doc = " let mut intersection = a.intersection(&b);"] # [doc = " ```"] # [must_use = "this returns the intersection as an iterator, \
              without modifying either input set"] # [stable (feature = "rust1" , since = "1.0.0")] pub struct Intersection < 'a , T : 'a , S : 'a > { iter : Iter < 'a , T > , other : & 'a HashSet < T , S > , }
};
}
