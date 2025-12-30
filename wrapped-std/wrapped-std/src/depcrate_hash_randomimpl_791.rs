// Generated macro for impl_791 (impl)
macro_rules! Depcrate_hash_randomimpl_791 {
() => {
// Module: crate::hash::random
// Provides: {"impl_791"}
// Dependencies: {}
impl RandomState { # [doc = " Constructs a new `RandomState` that is initialized with random keys."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::hash::RandomState;"] # [doc = ""] # [doc = " let s = RandomState::new();"] # [doc = " ```"] # [inline] # [allow (deprecated)] # [must_use] # [stable (feature = "hashmap_build_hasher" , since = "1.7.0")] pub fn new () -> RandomState { thread_local ! (static KEYS : Cell < (u64 , u64) > = { Cell :: new (hashmap_random_keys ()) }) ; KEYS . with (| keys | { let (k0 , k1) = keys . get () ; keys . set ((k0 . wrapping_add (1) , k1)) ; RandomState { k0 , k1 } }) } }
};
}
