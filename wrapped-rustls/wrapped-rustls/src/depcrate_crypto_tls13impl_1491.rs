// Generated macro for impl_1491 (impl)
macro_rules! Depcrate_crypto_tls13impl_1491 {
() => {
// Module: crate::crypto::tls13
// Provides: {"impl_1491"}
// Dependencies: {}
impl HkdfExpanderUsingHmac { fn expand_unchecked (& self , info : & [& [u8]] , output : & mut [u8]) { let mut term = hmac :: Tag :: new (b"") ; for (n , chunk) in output . chunks_mut (self . 0 . tag_len ()) . enumerate () { term = self . 0 . sign_concat (term . as_ref () , info , & [(n + 1) as u8]) ; chunk . copy_from_slice (& term . as_ref () [.. chunk . len ()]) ; } } }
};
}
