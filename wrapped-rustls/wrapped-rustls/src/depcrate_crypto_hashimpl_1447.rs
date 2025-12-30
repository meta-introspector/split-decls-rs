// Generated macro for impl_1447 (impl)
macro_rules! Depcrate_crypto_hashimpl_1447 {
() => {
// Module: crate::crypto::hash
// Provides: {"impl_1447"}
// Dependencies: {}
impl Output { # [doc = " Build a `hash::Output` from a slice of no more than `Output::MAX_LEN` bytes."] pub fn new (bytes : & [u8]) -> Self { let mut output = Self { buf : [0u8 ; Self :: MAX_LEN] , used : bytes . len () , } ; debug_assert ! (bytes . len () <= Self :: MAX_LEN) ; output . buf [.. bytes . len ()] . copy_from_slice (bytes) ; output } # [doc = " Maximum supported hash output size: supports up to SHA512."] pub const MAX_LEN : usize = 64 ; }
};
}
