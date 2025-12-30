// Generated macro for impl_1501 (impl)
macro_rules! Depcrate_crypto_tls13impl_1501 {
() => {
// Module: crate::crypto::tls13
// Provides: {"impl_1501"}
// Dependencies: {}
impl OkmBlock { # [doc = " Build a single OKM block by copying a byte slice."] # [doc = ""] # [doc = " The slice can be up to [`OkmBlock::MAX_LEN`] bytes in length."] pub fn new (bytes : & [u8]) -> Self { let mut tag = Self { buf : [0u8 ; Self :: MAX_LEN] , used : bytes . len () , } ; tag . buf [.. bytes . len ()] . copy_from_slice (bytes) ; tag } # [doc = " Maximum supported HMAC tag size: supports up to SHA512."] pub const MAX_LEN : usize = 64 ; }
};
}
