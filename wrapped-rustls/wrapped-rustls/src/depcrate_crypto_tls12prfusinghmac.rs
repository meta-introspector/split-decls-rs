// Generated macro for PrfUsingHmac (struct)
macro_rules! Depcrate_crypto_tls12PrfUsingHmac {
() => {
// Module: crate::crypto::tls12
// Provides: {"PrfUsingHmac"}
// Dependencies: {}
# [doc = " Implements [`Prf`] using a [`hmac::Hmac`]."] # [expect (clippy :: exhaustive_structs)] pub struct PrfUsingHmac < 'a > (pub & 'a dyn hmac :: Hmac) ;
};
}
