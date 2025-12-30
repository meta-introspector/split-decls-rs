// Generated macro for impl_349 (impl)
macro_rules! Depcrate_collections_hash_mapimpl_349 {
() => {
// Module: crate::collections::hash::map
// Provides: {"impl_349"}
// Dependencies: {}
# [stable (feature = "std_collections_from_array" , since = "1.56.0")] impl < K , V , const N : usize > From < [(K , V) ; N] > for HashMap < K , V , RandomState > where K : Eq + Hash , { # [doc = " Converts a `[(K, V); N]` into a `HashMap<K, V>`."] # [doc = ""] # [doc = " If any entries in the array have equal keys,"] # [doc = " all but one of the corresponding values will be dropped."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::collections::HashMap;"] # [doc = ""] # [doc = " let map1 = HashMap::from([(1, 2), (3, 4)]);"] # [doc = " let map2: HashMap<_, _> = [(1, 2), (3, 4)].into();"] # [doc = " assert_eq!(map1, map2);"] # [doc = " ```"] fn from (arr : [(K , V) ; N]) -> Self { Self :: from_iter (arr) } }
};
}
