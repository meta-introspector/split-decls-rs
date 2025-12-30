// Generated macro for RawHasher (struct)
macro_rules! Depcrate_xxhash3_128RawHasher {
() => {
// Module: crate::xxhash3_128
// Provides: {"RawHasher"}
// Dependencies: {}
# [derive (Clone)] # [doc = " A lower-level interface for computing a hash from streaming data."] # [doc = ""] # [doc = " The algorithm requires a secret which can be a reasonably large"] # [doc = " piece of data. [`Hasher`][] makes one concrete implementation"] # [doc = " decision that uses dynamic memory allocation, but specialized"] # [doc = " usages may desire more flexibility. This type, combined with"] # [doc = " [`SecretBuffer`][], offer that flexibility at the cost of a"] # [doc = " generic type."] pub struct RawHasher < S > (RawHasherCore < S >) ;
};
}
