// Generated macro for impl_1494 (impl)
macro_rules! Depcrate_crypto_tls13impl_1494 {
() => {
// Module: crate::crypto::tls13
// Provides: {"impl_1494"}
// Dependencies: {}
impl Hkdf for HkdfUsingHmac < '_ > { fn extract_from_zero_ikm (& self , salt : Option < & [u8] >) -> Box < dyn HkdfExpander > { let zeroes = [0u8 ; hmac :: Tag :: MAX_LEN] ; Box :: new (HkdfExpanderUsingHmac (self . 0 . with_key (& self . extract_prk_from_secret (salt , & zeroes [.. self . 0 . hash_output_len ()]) ,))) } fn extract_from_secret (& self , salt : Option < & [u8] > , secret : & [u8]) -> Box < dyn HkdfExpander > { Box :: new (HkdfExpanderUsingHmac (self . 0 . with_key (& self . extract_prk_from_secret (salt , secret)) ,)) } fn expander_for_okm (& self , okm : & OkmBlock) -> Box < dyn HkdfExpander > { Box :: new (HkdfExpanderUsingHmac (self . 0 . with_key (okm . as_ref ()))) } fn hmac_sign (& self , key : & OkmBlock , message : & [u8]) -> hmac :: Tag { self . 0 . with_key (key . as_ref ()) . sign (& [message]) } }
};
}
