// Generated macro for impl_1492 (impl)
macro_rules! Depcrate_crypto_tls13impl_1492 {
() => {
// Module: crate::crypto::tls13
// Provides: {"impl_1492"}
// Dependencies: {}
impl HkdfExpander for HkdfExpanderUsingHmac { fn expand_slice (& self , info : & [& [u8]] , output : & mut [u8]) -> Result < () , OutputLengthError > { if output . len () > 255 * self . 0 . tag_len () { return Err (OutputLengthError) ; } self . expand_unchecked (info , output) ; Ok (()) } fn expand_block (& self , info : & [& [u8]]) -> OkmBlock { let mut tag = [0u8 ; hmac :: Tag :: MAX_LEN] ; let reduced_tag = & mut tag [.. self . 0 . tag_len ()] ; self . expand_unchecked (info , reduced_tag) ; OkmBlock :: new (reduced_tag) } fn hash_len (& self) -> usize { self . 0 . tag_len () } }
};
}
