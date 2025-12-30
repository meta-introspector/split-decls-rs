// Generated macro for hash (function)
macro_rules! Depcratehash {
() => {
// Module: crate
// Provides: {"hash"}
// Dependencies: {}
# [doc = " Return a Sha256 hash for the given data."] # [cfg_attr (target_os = "solana" , inline (always))] pub fn hash (val : & [u8]) -> Hash { hashv (& [val]) }
};
}
