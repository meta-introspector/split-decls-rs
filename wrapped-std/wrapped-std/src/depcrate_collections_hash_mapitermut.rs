// Generated macro for IterMut (struct)
macro_rules! Depcrate_collections_hash_mapIterMut {
() => {
// Module: crate::collections::hash::map
// Provides: {"IterMut"}
// Dependencies: {}
# [doc = " A mutable iterator over the entries of a `HashMap`."] # [doc = ""] # [doc = " This `struct` is created by the [`iter_mut`] method on [`HashMap`]. See its"] # [doc = " documentation for more."] # [doc = ""] # [doc = " [`iter_mut`]: HashMap::iter_mut"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use std::collections::HashMap;"] # [doc = ""] # [doc = " let mut map = HashMap::from(["] # [doc = "     (\"a\", 1),"] # [doc = " ]);"] # [doc = " let iter = map.iter_mut();"] # [doc = " ```"] # [stable (feature = "rust1" , since = "1.0.0")] # [cfg_attr (not (test) , rustc_diagnostic_item = "hashmap_iter_mut_ty")] pub struct IterMut < 'a , K : 'a , V : 'a > { base : base :: IterMut < 'a , K , V > , }
};
}
