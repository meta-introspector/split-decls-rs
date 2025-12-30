// Generated macro for impl_447 (impl)
macro_rules! Depcrate_collections_hash_setimpl_447 {
() => {
// Module: crate::collections::hash::set
// Provides: {"impl_447"}
// Dependencies: {}
impl < T > HashSet < T , RandomState > { # [doc = " Creates an empty `HashSet`."] # [doc = ""] # [doc = " The hash set is initially created with a capacity of 0, so it will not allocate until it"] # [doc = " is first inserted into."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::collections::HashSet;"] # [doc = " let set: HashSet<i32> = HashSet::new();"] # [doc = " ```"] # [inline] # [must_use] # [stable (feature = "rust1" , since = "1.0.0")] pub fn new () -> HashSet < T , RandomState > { Default :: default () } # [doc = " Creates an empty `HashSet` with at least the specified capacity."] # [doc = ""] # [doc = " The hash set will be able to hold at least `capacity` elements without"] # [doc = " reallocating. This method is allowed to allocate for more elements than"] # [doc = " `capacity`. If `capacity` is zero, the hash set will not allocate."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::collections::HashSet;"] # [doc = " let set: HashSet<i32> = HashSet::with_capacity(10);"] # [doc = " assert!(set.capacity() >= 10);"] # [doc = " ```"] # [inline] # [must_use] # [stable (feature = "rust1" , since = "1.0.0")] pub fn with_capacity (capacity : usize) -> HashSet < T , RandomState > { HashSet :: with_capacity_and_hasher (capacity , Default :: default ()) } }
};
}
