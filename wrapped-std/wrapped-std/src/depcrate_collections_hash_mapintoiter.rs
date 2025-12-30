// Generated macro for IntoIter (struct)
macro_rules! Depcrate_collections_hash_mapIntoIter {
() => {
// Module: crate::collections::hash::map
// Provides: {"IntoIter"}
// Dependencies: {}
# [doc = " An owning iterator over the entries of a `HashMap`."] # [doc = ""] # [doc = " This `struct` is created by the [`into_iter`] method on [`HashMap`]"] # [doc = " (provided by the [`IntoIterator`] trait). See its documentation for more."] # [doc = ""] # [doc = " [`into_iter`]: IntoIterator::into_iter"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use std::collections::HashMap;"] # [doc = ""] # [doc = " let map = HashMap::from(["] # [doc = "     (\"a\", 1),"] # [doc = " ]);"] # [doc = " let iter = map.into_iter();"] # [doc = " ```"] # [stable (feature = "rust1" , since = "1.0.0")] pub struct IntoIter < K , V > { base : base :: IntoIter < K , V > , }
};
}
