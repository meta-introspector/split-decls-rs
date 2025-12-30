// Generated macro for impl_84 (impl)
macro_rules! Depcrate_hash_indeximpl_84 {
() => {
// Module: crate::hash_index
// Provides: {"impl_84"}
// Dependencies: {}
impl < K , V > HashIndex < K , V , RandomState > where K : Eq + Hash , { # [doc = " Creates an empty default [`HashIndex`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use scc::HashIndex;"] # [doc = ""] # [doc = " let hashindex: HashIndex<u64, u32> = HashIndex::new();"] # [doc = ""] # [doc = " let result = hashindex.capacity();"] # [doc = " assert_eq!(result, 0);"] # [doc = " ```"] # [inline] # [must_use] pub fn new () -> Self { Self :: default () } # [doc = " Creates an empty [`HashIndex`] with the specified capacity."] # [doc = ""] # [doc = " The actual capacity is equal to or greater than `capacity` unless it is greater than"] # [doc = " `1 << (usize::BITS - 1)`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use scc::HashIndex;"] # [doc = ""] # [doc = " let hashindex: HashIndex<u64, u32> = HashIndex::with_capacity(1000);"] # [doc = ""] # [doc = " let result = hashindex.capacity();"] # [doc = " assert_eq!(result, 1024);"] # [doc = " ```"] # [inline] # [must_use] pub fn with_capacity (capacity : usize) -> Self { Self :: with_capacity_and_hasher (capacity , RandomState :: new ()) } }
};
}
