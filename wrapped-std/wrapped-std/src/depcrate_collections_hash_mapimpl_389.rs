// Generated macro for impl_389 (impl)
macro_rules! Depcrate_collections_hash_mapimpl_389 {
() => {
// Module: crate::collections::hash::map
// Provides: {"impl_389"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < K , V , S > IntoIterator for HashMap < K , V , S > { type Item = (K , V) ; type IntoIter = IntoIter < K , V > ; # [doc = " Creates a consuming iterator, that is, one that moves each key-value"] # [doc = " pair out of the map in arbitrary order. The map cannot be used after"] # [doc = " calling this."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::collections::HashMap;"] # [doc = ""] # [doc = " let map = HashMap::from(["] # [doc = "     (\"a\", 1),"] # [doc = "     (\"b\", 2),"] # [doc = "     (\"c\", 3),"] # [doc = " ]);"] # [doc = ""] # [doc = " // Not possible with .iter()"] # [doc = " let vec: Vec<(&str, i32)> = map.into_iter().collect();"] # [doc = " ```"] # [inline] # [rustc_lint_query_instability] fn into_iter (self) -> IntoIter < K , V > { IntoIter { base : self . base . into_iter () } } }
};
}
