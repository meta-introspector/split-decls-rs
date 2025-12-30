// Generated macro for IntoIter (struct)
macro_rules! Depcrate_collections_hash_setIntoIter {
() => {
// Module: crate::collections::hash::set
// Provides: {"IntoIter"}
// Dependencies: {}
# [doc = " An owning iterator over the items of a `HashSet`."] # [doc = ""] # [doc = " This `struct` is created by the [`into_iter`] method on [`HashSet`]"] # [doc = " (provided by the [`IntoIterator`] trait). See its documentation for more."] # [doc = ""] # [doc = " [`into_iter`]: IntoIterator::into_iter"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::collections::HashSet;"] # [doc = ""] # [doc = " let a = HashSet::from([1, 2, 3]);"] # [doc = ""] # [doc = " let mut iter = a.into_iter();"] # [doc = " ```"] # [stable (feature = "rust1" , since = "1.0.0")] pub struct IntoIter < K > { base : base :: IntoIter < K > , }
};
}
