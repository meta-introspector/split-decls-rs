// Generated macro for DefaultHasher (struct)
macro_rules! Depcrate_hash_randomDefaultHasher {
() => {
// Module: crate::hash::random
// Provides: {"DefaultHasher"}
// Dependencies: {}
# [doc = " The default [`Hasher`] used by [`RandomState`]."] # [doc = ""] # [doc = " The internal algorithm is not specified, and so it and its hashes should"] # [doc = " not be relied upon over releases."] # [allow (deprecated)] # [derive (Clone , Debug)] # [stable (feature = "hashmap_build_hasher" , since = "1.7.0")] pub struct DefaultHasher (SipHasher13) ;
};
}
