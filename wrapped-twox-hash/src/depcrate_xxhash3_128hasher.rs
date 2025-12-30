// Generated macro for Hasher (struct)
macro_rules! Depcrate_xxhash3_128Hasher {
() => {
// Module: crate::xxhash3_128
// Provides: {"Hasher"}
// Dependencies: {}
# [doc = " Calculates the 128-bit hash."] # [doc = ""] # [doc = " This type does not implement [`std::hash::Hasher`] as that trait"] # [doc = " requires a 64-bit result while this computes a 128-bit result."] # [derive (Clone)] pub struct Hasher { # [cfg (feature = "alloc")] inner : AllocRawHasher , _private : () , }
};
}
