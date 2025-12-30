// Generated macro for impl_137 (impl)
macro_rules! Depcrate_hash_mapimpl_137 {
() => {
// Module: crate::hash_map
// Provides: {"impl_137"}
// Dependencies: {}
impl < K , V , H > Default for HashMap < K , V , H > where H : BuildHasher + Default , { # [doc = " Creates an empty default [`HashMap`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use scc::HashMap;"] # [doc = ""] # [doc = " let hashmap: HashMap<u64, u32> = HashMap::default();"] # [doc = ""] # [doc = " let result = hashmap.capacity();"] # [doc = " assert_eq!(result, 0);"] # [doc = " ```"] # [inline] fn default () -> Self { Self :: with_hasher (H :: default ()) } }
};
}
