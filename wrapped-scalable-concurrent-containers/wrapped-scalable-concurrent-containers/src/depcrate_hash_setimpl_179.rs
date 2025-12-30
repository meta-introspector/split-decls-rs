// Generated macro for impl_179 (impl)
macro_rules! Depcrate_hash_setimpl_179 {
() => {
// Module: crate::hash_set
// Provides: {"impl_179"}
// Dependencies: {}
impl < K : Eq + Hash > HashSet < K , RandomState > { # [doc = " Creates an empty default [`HashSet`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use scc::HashSet;"] # [doc = ""] # [doc = " let hashset: HashSet<u64> = HashSet::new();"] # [doc = ""] # [doc = " let result = hashset.capacity();"] # [doc = " assert_eq!(result, 0);"] # [doc = " ```"] # [inline] # [must_use] pub fn new () -> Self { Self :: default () } # [doc = " Creates an empty [`HashSet`] with the specified capacity."] # [doc = ""] # [doc = " The actual capacity is equal to or greater than `capacity` unless it is greater than"] # [doc = " `1 << (usize::BITS - 1)`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use scc::HashSet;"] # [doc = ""] # [doc = " let hashset: HashSet<u64> = HashSet::with_capacity(1000);"] # [doc = ""] # [doc = " let result = hashset.capacity();"] # [doc = " assert_eq!(result, 1024);"] # [doc = " ```"] # [inline] # [must_use] pub fn with_capacity (capacity : usize) -> Self { Self { map : HashMap :: with_capacity (capacity) , } } }
};
}
