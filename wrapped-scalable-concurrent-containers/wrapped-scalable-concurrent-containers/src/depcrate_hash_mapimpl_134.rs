// Generated macro for impl_134 (impl)
macro_rules! Depcrate_hash_mapimpl_134 {
() => {
// Module: crate::hash_map
// Provides: {"impl_134"}
// Dependencies: {}
impl < K , V > HashMap < K , V , RandomState > { # [doc = " Creates an empty default [`HashMap`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use scc::HashMap;"] # [doc = ""] # [doc = " let hashmap: HashMap<u64, u32> = HashMap::new();"] # [doc = ""] # [doc = " let result = hashmap.capacity();"] # [doc = " assert_eq!(result, 0);"] # [doc = " ```"] # [inline] # [must_use] pub fn new () -> Self { Self :: default () } # [doc = " Creates an empty [`HashMap`] with the specified capacity."] # [doc = ""] # [doc = " The actual capacity is equal to or greater than `capacity` unless it is greater than"] # [doc = " `1 << (usize::BITS - 1)`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use scc::HashMap;"] # [doc = ""] # [doc = " let hashmap: HashMap<u64, u32> = HashMap::with_capacity(1000);"] # [doc = ""] # [doc = " let result = hashmap.capacity();"] # [doc = " assert_eq!(result, 1024);"] # [doc = " ```"] # [inline] # [must_use] pub fn with_capacity (capacity : usize) -> Self { Self :: with_capacity_and_hasher (capacity , RandomState :: new ()) } }
};
}
