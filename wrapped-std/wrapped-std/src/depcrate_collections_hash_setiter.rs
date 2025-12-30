// Generated macro for Iter (struct)
macro_rules! Depcrate_collections_hash_setIter {
() => {
// Module: crate::collections::hash::set
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " An iterator over the items of a `HashSet`."] # [doc = ""] # [doc = " This `struct` is created by the [`iter`] method on [`HashSet`]."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " [`iter`]: HashSet::iter"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::collections::HashSet;"] # [doc = ""] # [doc = " let a = HashSet::from([1, 2, 3]);"] # [doc = ""] # [doc = " let mut iter = a.iter();"] # [doc = " ```"] # [stable (feature = "rust1" , since = "1.0.0")] # [cfg_attr (not (test) , rustc_diagnostic_item = "hashset_iter_ty")] pub struct Iter < 'a , K : 'a > { base : base :: Iter < 'a , K > , }
};
}
