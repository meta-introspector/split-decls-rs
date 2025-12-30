// Generated macro for compress256 (function)
macro_rules! Depcrate_sha256compress256 {
() => {
// Module: crate::sha256
// Provides: {"compress256"}
// Dependencies: {}
# [doc = " Raw SHA-256 compression function."] # [doc = ""] # [doc = " This is a low-level \"hazmat\" API which provides direct access to the core"] # [doc = " functionality of SHA-256."] pub fn compress256 (state : & mut [u32 ; 8] , blocks : & [[u8 ; 64]]) { compress (state , blocks) }
};
}
