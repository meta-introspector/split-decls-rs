// Generated macro for impl_792 (impl)
macro_rules! Depcrate_hash_randomimpl_792 {
() => {
// Module: crate::hash::random
// Provides: {"impl_792"}
// Dependencies: {}
# [stable (feature = "hashmap_build_hasher" , since = "1.7.0")] impl BuildHasher for RandomState { type Hasher = DefaultHasher ; # [inline] # [allow (deprecated)] fn build_hasher (& self) -> DefaultHasher { DefaultHasher (SipHasher13 :: new_with_keys (self . k0 , self . k1)) } }
};
}
