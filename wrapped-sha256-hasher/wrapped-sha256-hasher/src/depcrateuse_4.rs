// Generated macro for use_4 (use)
macro_rules! Depcrateuse_4 {
() => {
// Module: crate
// Provides: {"use_4"}
// Dependencies: {}
# [cfg (all (feature = "sha2" , not (any (target_os = "solana" , target_arch = "bpf"))))] use { sha2 :: { Digest , Sha256 } , solana_hash :: HASH_BYTES , } ;
};
}
