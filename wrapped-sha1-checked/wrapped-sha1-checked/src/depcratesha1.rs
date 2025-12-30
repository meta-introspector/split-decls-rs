// Generated macro for Sha1 (struct)
macro_rules! DepcrateSha1 {
() => {
// Module: crate
// Provides: {"Sha1"}
// Dependencies: {}
# [doc = " SHA-1 collision detection hasher state."] # [derive (Clone)] pub struct Sha1 { h : [u32 ; STATE_LEN] , block_len : u64 , detection : Option < DetectionState > , buffer : BlockBuffer < U64 , Eager > , }
};
}
