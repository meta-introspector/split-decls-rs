// Generated macro for Hash (trait)
macro_rules! Depcrate_crypto_hashHash {
() => {
// Module: crate::crypto::hash
// Provides: {"Hash"}
// Dependencies: {}
# [doc = " Describes a single cryptographic hash function."] # [doc = ""] # [doc = " This interface can do both one-shot and incremental hashing, using"] # [doc = " [`Hash::hash()`] and [`Hash::start()`] respectively."] pub trait Hash : Send + Sync { # [doc = " Start an incremental hash computation."] fn start (& self) -> Box < dyn Context > ; # [doc = " Return the output of this hash function with input `data`."] fn hash (& self , data : & [u8]) -> Output ; # [doc = " The length in bytes of this hash function's output."] fn output_len (& self) -> usize ; # [doc = " Which hash function this is, eg, `HashAlgorithm::SHA256`."] fn algorithm (& self) -> HashAlgorithm ; # [doc = " Return `true` if this is backed by a FIPS-approved implementation."] fn fips (& self) -> bool { false } }
};
}
