// Generated macro for impl_340 (impl)
macro_rules! Depcrate_collections_hash_mapimpl_340 {
() => {
// Module: crate::collections::hash::map
// Provides: {"impl_340"}
// Dependencies: {}
impl < K , V > HashMap < K , V , RandomState > { # [doc = " Creates an empty `HashMap`."] # [doc = ""] # [doc = " The hash map is initially created with a capacity of 0, so it will not allocate until it"] # [doc = " is first inserted into."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::collections::HashMap;"] # [doc = " let mut map: HashMap<&str, i32> = HashMap::new();"] # [doc = " ```"] # [inline] # [must_use] # [stable (feature = "rust1" , since = "1.0.0")] pub fn new () -> HashMap < K , V , RandomState > { Default :: default () } # [doc = " Creates an empty `HashMap` with at least the specified capacity."] # [doc = ""] # [doc = " The hash map will be able to hold at least `capacity` elements without"] # [doc = " reallocating. This method is allowed to allocate for more elements than"] # [doc = " `capacity`. If `capacity` is zero, the hash map will not allocate."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::collections::HashMap;"] # [doc = " let mut map: HashMap<&str, i32> = HashMap::with_capacity(10);"] # [doc = " ```"] # [inline] # [must_use] # [stable (feature = "rust1" , since = "1.0.0")] pub fn with_capacity (capacity : usize) -> HashMap < K , V , RandomState > { HashMap :: with_capacity_and_hasher (capacity , Default :: default ()) } }
};
}
