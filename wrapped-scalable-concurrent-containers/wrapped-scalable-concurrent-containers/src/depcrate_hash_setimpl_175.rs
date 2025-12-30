// Generated macro for impl_175 (impl)
macro_rules! Depcrate_hash_setimpl_175 {
() => {
// Module: crate::hash_set
// Provides: {"impl_175"}
// Dependencies: {}
impl < K , H > HashSet < K , H > where H : BuildHasher , { # [doc = " Creates an empty [`HashSet`] with the given [`BuildHasher`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use scc::HashSet;"] # [doc = " use std::collections::hash_map::RandomState;"] # [doc = ""] # [doc = " let hashset: HashSet<u64, RandomState> = HashSet::with_hasher(RandomState::new());"] # [doc = " ```"] # [cfg (not (feature = "loom"))] # [inline] pub const fn with_hasher (build_hasher : H) -> Self { Self { map : HashMap :: with_hasher (build_hasher) , } } # [doc = " Creates an empty [`HashSet`] with the given [`BuildHasher`]."] # [cfg (feature = "loom")] # [inline] pub fn with_hasher (build_hasher : H) -> Self { Self { map : HashMap :: with_hasher (build_hasher) , } } # [doc = " Creates an empty [`HashSet`] with the specified capacity and [`BuildHasher`]."] # [doc = ""] # [doc = " The actual capacity is equal to or greater than `capacity` unless it is greater than"] # [doc = " `1 << (usize::BITS - 1)`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use scc::HashSet;"] # [doc = " use std::collections::hash_map::RandomState;"] # [doc = ""] # [doc = " let hashset: HashSet<u64, RandomState> ="] # [doc = "     HashSet::with_capacity_and_hasher(1000, RandomState::new());"] # [doc = ""] # [doc = " let result = hashset.capacity();"] # [doc = " assert_eq!(result, 1024);"] # [doc = " ```"] # [inline] pub fn with_capacity_and_hasher (capacity : usize , build_hasher : H) -> Self { Self { map : HashMap :: with_capacity_and_hasher (capacity , build_hasher) , } } }
};
}
