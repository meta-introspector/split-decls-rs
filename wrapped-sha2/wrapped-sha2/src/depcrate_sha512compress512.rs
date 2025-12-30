// Generated macro for compress512 (function)
macro_rules! Depcrate_sha512compress512 {
() => {
// Module: crate::sha512
// Provides: {"compress512"}
// Dependencies: {}
# [doc = " Raw SHA-512 compression function."] # [doc = ""] # [doc = " This is a low-level \"hazmat\" API which provides direct access to the core"] # [doc = " functionality of SHA-512."] pub fn compress512 (state : & mut [u64 ; 8] , blocks : & [[u8 ; 128]]) { compress (state , blocks) }
};
}
