// Generated macro for HkdfUsingHmac (struct)
macro_rules! Depcrate_crypto_tls13HkdfUsingHmac {
() => {
// Module: crate::crypto::tls13
// Provides: {"HkdfUsingHmac"}
// Dependencies: {}
# [doc = " Implementation of `Hkdf` (and thence `HkdfExpander`) via `hmac::Hmac`."] # [expect (clippy :: exhaustive_structs)] pub struct HkdfUsingHmac < 'a > (pub & 'a dyn hmac :: Hmac) ;
};
}
