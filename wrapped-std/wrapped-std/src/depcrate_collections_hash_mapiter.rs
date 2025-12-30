// Generated macro for Iter (struct)
macro_rules! Depcrate_collections_hash_mapIter {
() => {
// Module: crate::collections::hash::map
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " An iterator over the entries of a `HashMap`."] # [doc = ""] # [doc = " This `struct` is created by the [`iter`] method on [`HashMap`]. See its"] # [doc = " documentation for more."] # [doc = ""] # [doc = " [`iter`]: HashMap::iter"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use std::collections::HashMap;"] # [doc = ""] # [doc = " let map = HashMap::from(["] # [doc = "     (\"a\", 1),"] # [doc = " ]);"] # [doc = " let iter = map.iter();"] # [doc = " ```"] # [stable (feature = "rust1" , since = "1.0.0")] # [cfg_attr (not (test) , rustc_diagnostic_item = "hashmap_iter_ty")] pub struct Iter < 'a , K : 'a , V : 'a > { base : base :: Iter < 'a , K , V > , }
};
}
