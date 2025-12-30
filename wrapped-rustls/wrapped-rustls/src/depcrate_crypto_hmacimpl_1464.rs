// Generated macro for impl_1464 (impl)
macro_rules! Depcrate_crypto_hmacimpl_1464 {
() => {
// Module: crate::crypto::hmac
// Provides: {"impl_1464"}
// Dependencies: {}
impl PublicTag { # [doc = " Build a tag by copying a byte slice."] # [doc = ""] # [doc = " The slice can be up to [`Tag::MAX_LEN`] bytes in length."] pub (crate) fn new (bytes : & [u8]) -> Self { let mut tag = Self { buf : [0u8 ; Tag :: MAX_LEN] , used : bytes . len () , } ; tag . buf [.. bytes . len ()] . copy_from_slice (bytes) ; tag } }
};
}
