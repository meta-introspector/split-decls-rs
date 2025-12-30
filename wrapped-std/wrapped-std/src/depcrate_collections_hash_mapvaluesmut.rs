// Generated macro for ValuesMut (struct)
macro_rules! Depcrate_collections_hash_mapValuesMut {
() => {
// Module: crate::collections::hash::map
// Provides: {"ValuesMut"}
// Dependencies: {}
# [doc = " A mutable iterator over the values of a `HashMap`."] # [doc = ""] # [doc = " This `struct` is created by the [`values_mut`] method on [`HashMap`]. See its"] # [doc = " documentation for more."] # [doc = ""] # [doc = " [`values_mut`]: HashMap::values_mut"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use std::collections::HashMap;"] # [doc = ""] # [doc = " let mut map = HashMap::from(["] # [doc = "     (\"a\", 1),"] # [doc = " ]);"] # [doc = " let iter_values = map.values_mut();"] # [doc = " ```"] # [stable (feature = "map_values_mut" , since = "1.10.0")] # [cfg_attr (not (test) , rustc_diagnostic_item = "hashmap_values_mut_ty")] pub struct ValuesMut < 'a , K : 'a , V : 'a > { inner : IterMut < 'a , K , V > , }
};
}
