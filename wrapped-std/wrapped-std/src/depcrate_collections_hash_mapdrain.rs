// Generated macro for Drain (struct)
macro_rules! Depcrate_collections_hash_mapDrain {
() => {
// Module: crate::collections::hash::map
// Provides: {"Drain"}
// Dependencies: {}
# [doc = " A draining iterator over the entries of a `HashMap`."] # [doc = ""] # [doc = " This `struct` is created by the [`drain`] method on [`HashMap`]. See its"] # [doc = " documentation for more."] # [doc = ""] # [doc = " [`drain`]: HashMap::drain"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use std::collections::HashMap;"] # [doc = ""] # [doc = " let mut map = HashMap::from(["] # [doc = "     (\"a\", 1),"] # [doc = " ]);"] # [doc = " let iter = map.drain();"] # [doc = " ```"] # [stable (feature = "drain" , since = "1.6.0")] # [cfg_attr (not (test) , rustc_diagnostic_item = "hashmap_drain_ty")] pub struct Drain < 'a , K : 'a , V : 'a > { base : base :: Drain < 'a , K , V > , }
};
}
