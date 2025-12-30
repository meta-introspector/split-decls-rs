// Generated macro for RandomState (struct)
macro_rules! Depcrate_hash_randomRandomState {
() => {
// Module: crate::hash::random
// Provides: {"RandomState"}
// Dependencies: {}
# [doc = " `RandomState` is the default state for [`HashMap`] types."] # [doc = ""] # [doc = " A particular instance `RandomState` will create the same instances of"] # [doc = " [`Hasher`], but the hashers created by two different `RandomState`"] # [doc = " instances are unlikely to produce the same result for the same values."] # [doc = ""] # [doc = " [`HashMap`]: crate::collections::HashMap"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::collections::HashMap;"] # [doc = " use std::hash::RandomState;"] # [doc = ""] # [doc = " let s = RandomState::new();"] # [doc = " let mut map = HashMap::with_hasher(s);"] # [doc = " map.insert(1, 2);"] # [doc = " ```"] # [stable (feature = "hashmap_build_hasher" , since = "1.7.0")] # [derive (Clone)] pub struct RandomState { k0 : u64 , k1 : u64 , }
};
}
