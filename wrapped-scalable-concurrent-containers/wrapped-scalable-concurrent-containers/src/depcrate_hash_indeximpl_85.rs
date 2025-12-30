// Generated macro for impl_85 (impl)
macro_rules! Depcrate_hash_indeximpl_85 {
() => {
// Module: crate::hash_index
// Provides: {"impl_85"}
// Dependencies: {}
impl < K , V , H > Default for HashIndex < K , V , H > where H : BuildHasher + Default , { # [doc = " Creates an empty default [`HashIndex`]."] # [doc = ""] # [doc = " The default capacity is `64`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use scc::HashIndex;"] # [doc = ""] # [doc = " let hashindex: HashIndex<u64, u32> = HashIndex::default();"] # [doc = ""] # [doc = " let result = hashindex.capacity();"] # [doc = " assert_eq!(result, 0);"] # [doc = " ```"] # [inline] fn default () -> Self { Self :: with_hasher (H :: default ()) } }
};
}
