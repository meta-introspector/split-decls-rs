// Generated macro for impl_794 (impl)
macro_rules! Depcrate_hash_randomimpl_794 {
() => {
// Module: crate::hash::random
// Provides: {"impl_794"}
// Dependencies: {}
impl DefaultHasher { # [doc = " Creates a new `DefaultHasher`."] # [doc = ""] # [doc = " This hasher is not guaranteed to be the same as all other"] # [doc = " `DefaultHasher` instances, but is the same as all other `DefaultHasher`"] # [doc = " instances created through `new` or `default`."] # [stable (feature = "hashmap_default_hasher" , since = "1.13.0")] # [inline] # [allow (deprecated)] # [must_use] pub fn new () -> DefaultHasher { DefaultHasher (SipHasher13 :: new_with_keys (0 , 0)) } }
};
}
