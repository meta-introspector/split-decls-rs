// Generated macro for impl_180 (impl)
macro_rules! Depcrate_hash_setimpl_180 {
() => {
// Module: crate::hash_set
// Provides: {"impl_180"}
// Dependencies: {}
impl < K , H > Default for HashSet < K , H > where H : BuildHasher + Default , { # [doc = " Creates an empty default [`HashSet`]."] # [doc = ""] # [doc = " The default capacity is `0`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use scc::HashSet;"] # [doc = ""] # [doc = " let hashset: HashSet<u64> = HashSet::default();"] # [doc = ""] # [doc = " let result = hashset.capacity();"] # [doc = " assert_eq!(result, 0);"] # [doc = " ```"] # [inline] fn default () -> Self { Self { map : HashMap :: default () , } } }
};
}
