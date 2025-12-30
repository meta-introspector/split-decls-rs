// Generated macro for IntoKeys (struct)
macro_rules! Depcrate_collections_hash_mapIntoKeys {
() => {
// Module: crate::collections::hash::map
// Provides: {"IntoKeys"}
// Dependencies: {}
# [doc = " An owning iterator over the keys of a `HashMap`."] # [doc = ""] # [doc = " This `struct` is created by the [`into_keys`] method on [`HashMap`]."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " [`into_keys`]: HashMap::into_keys"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use std::collections::HashMap;"] # [doc = ""] # [doc = " let map = HashMap::from(["] # [doc = "     (\"a\", 1),"] # [doc = " ]);"] # [doc = " let iter_keys = map.into_keys();"] # [doc = " ```"] # [stable (feature = "map_into_keys_values" , since = "1.54.0")] pub struct IntoKeys < K , V > { inner : IntoIter < K , V > , }
};
}
