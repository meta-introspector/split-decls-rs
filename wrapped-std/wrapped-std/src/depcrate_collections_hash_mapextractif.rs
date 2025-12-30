// Generated macro for ExtractIf (struct)
macro_rules! Depcrate_collections_hash_mapExtractIf {
() => {
// Module: crate::collections::hash::map
// Provides: {"ExtractIf"}
// Dependencies: {}
# [doc = " A draining, filtering iterator over the entries of a `HashMap`."] # [doc = ""] # [doc = " This `struct` is created by the [`extract_if`] method on [`HashMap`]."] # [doc = ""] # [doc = " [`extract_if`]: HashMap::extract_if"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use std::collections::HashMap;"] # [doc = ""] # [doc = " let mut map = HashMap::from(["] # [doc = "     (\"a\", 1),"] # [doc = " ]);"] # [doc = " let iter = map.extract_if(|_k, v| *v % 2 == 0);"] # [doc = " ```"] # [stable (feature = "hash_extract_if" , since = "1.88.0")] # [must_use = "iterators are lazy and do nothing unless consumed"] pub struct ExtractIf < 'a , K , V , F > { base : base :: ExtractIf < 'a , K , V , F > , }
};
}
