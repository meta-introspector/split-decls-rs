// Generated macro for HkdfPrkExtract (trait)
macro_rules! Depcrate_crypto_tls13HkdfPrkExtract {
() => {
// Module: crate::crypto::tls13
// Provides: {"HkdfPrkExtract"}
// Dependencies: {}
# [doc = " An extended HKDF implementation that supports directly extracting a pseudo-random key (PRK)."] # [doc = ""] # [doc = " The base [`Hkdf`] trait is tailored to the needs of TLS 1.3, where all extracted PRKs"] # [doc = " are expanded as-is, and so can be safely encapsulated without exposing the caller"] # [doc = " to the key material."] # [doc = ""] # [doc = " In other contexts (for example, hybrid public key encryption (HPKE)) it may be necessary"] # [doc = " to use the extracted PRK directly for purposes other than an immediate expansion."] # [doc = " This trait can be implemented to offer this functionality when it is required."] pub (crate) trait HkdfPrkExtract : Hkdf { # [doc = " `HKDF-Extract(salt, secret)`"] # [doc = ""] # [doc = " A `salt` of `None` should be treated as a sequence of `HashLen` zero bytes."] # [doc = ""] # [doc = " In most cases you should prefer [`Hkdf::extract_from_secret`] and using the"] # [doc = " returned [HkdfExpander] instead of handling the PRK directly."] fn extract_prk_from_secret (& self , salt : Option < & [u8] > , secret : & [u8]) -> Vec < u8 > ; }
};
}
