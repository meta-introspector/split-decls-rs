// Generated macro for IntoValues (struct)
macro_rules! Depcrate_collections_hash_mapIntoValues {
() => {
// Module: crate::collections::hash::map
// Provides: {"IntoValues"}
// Dependencies: {}
# [doc = " An owning iterator over the values of a `HashMap`."] # [doc = ""] # [doc = " This `struct` is created by the [`into_values`] method on [`HashMap`]."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " [`into_values`]: HashMap::into_values"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use std::collections::HashMap;"] # [doc = ""] # [doc = " let map = HashMap::from(["] # [doc = "     (\"a\", 1),"] # [doc = " ]);"] # [doc = " let iter_keys = map.into_values();"] # [doc = " ```"] # [stable (feature = "map_into_keys_values" , since = "1.54.0")] pub struct IntoValues < K , V > { inner : IntoIter < K , V > , }
};
}
