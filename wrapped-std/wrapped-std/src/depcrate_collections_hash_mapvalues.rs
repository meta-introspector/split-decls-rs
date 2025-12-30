// Generated macro for Values (struct)
macro_rules! Depcrate_collections_hash_mapValues {
() => {
// Module: crate::collections::hash::map
// Provides: {"Values"}
// Dependencies: {}
# [doc = " An iterator over the values of a `HashMap`."] # [doc = ""] # [doc = " This `struct` is created by the [`values`] method on [`HashMap`]. See its"] # [doc = " documentation for more."] # [doc = ""] # [doc = " [`values`]: HashMap::values"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use std::collections::HashMap;"] # [doc = ""] # [doc = " let map = HashMap::from(["] # [doc = "     (\"a\", 1),"] # [doc = " ]);"] # [doc = " let iter_values = map.values();"] # [doc = " ```"] # [stable (feature = "rust1" , since = "1.0.0")] # [cfg_attr (not (test) , rustc_diagnostic_item = "hashmap_values_ty")] pub struct Values < 'a , K : 'a , V : 'a > { inner : Iter < 'a , K , V > , }
};
}
