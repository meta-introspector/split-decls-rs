// Generated macro for Keys (struct)
macro_rules! Depcrate_collections_hash_mapKeys {
() => {
// Module: crate::collections::hash::map
// Provides: {"Keys"}
// Dependencies: {}
# [doc = " An iterator over the keys of a `HashMap`."] # [doc = ""] # [doc = " This `struct` is created by the [`keys`] method on [`HashMap`]. See its"] # [doc = " documentation for more."] # [doc = ""] # [doc = " [`keys`]: HashMap::keys"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use std::collections::HashMap;"] # [doc = ""] # [doc = " let map = HashMap::from(["] # [doc = "     (\"a\", 1),"] # [doc = " ]);"] # [doc = " let iter_keys = map.keys();"] # [doc = " ```"] # [stable (feature = "rust1" , since = "1.0.0")] # [cfg_attr (not (test) , rustc_diagnostic_item = "hashmap_keys_ty")] pub struct Keys < 'a , K : 'a , V : 'a > { inner : Iter < 'a , K , V > , }
};
}
