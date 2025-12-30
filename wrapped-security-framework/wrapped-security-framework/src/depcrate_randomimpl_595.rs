// Generated macro for impl_595 (impl)
macro_rules! Depcrate_randomimpl_595 {
() => {
// Module: crate::random
// Provides: {"impl_595"}
// Dependencies: {}
impl SecRandom { # [doc = " Fills the buffer with cryptographically secure random bytes."] pub fn copy_bytes (& self , buf : & mut [u8]) -> io :: Result < () > { if unsafe { SecRandomCopyBytes (self . 0 , buf . len () , buf . as_mut_ptr () . cast ()) } == 0 { Ok (()) } else { Err (io :: Error :: last_os_error ()) } } }
};
}
