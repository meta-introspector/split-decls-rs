// Generated macro for Hasher (struct)
macro_rules! DepcrateHasher {
() => {
// Module: crate
// Provides: {"Hasher"}
// Dependencies: {}
# [cfg (all (feature = "sha2" , not (any (target_os = "solana" , target_arch = "bpf"))))] # [derive (Clone , Default)] pub struct Hasher { hasher : Sha256 , }
};
}
