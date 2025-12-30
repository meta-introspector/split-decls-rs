// Generated macro for Prf (trait)
macro_rules! Depcrate_crypto_tls12Prf {
() => {
// Module: crate::crypto::tls12
// Provides: {"Prf"}
// Dependencies: {}
# [doc = " An instantiation of the TLS1.2 PRF with a specific, implicit hash function."] # [doc = ""] # [doc = " See the definition in [RFC5246 section 5](https://www.rfc-editor.org/rfc/rfc5246#section-5)."] # [doc = ""] # [doc = " See [`PrfUsingHmac`] as a route to implementing this trait with just"] # [doc = " an implementation of [`hmac::Hmac`]."] pub trait Prf : Send + Sync { # [doc = " Computes `PRF(secret, label, seed)` using the secret from a completed key exchange."] # [doc = ""] # [doc = " Completes the given key exchange, and then uses the resulting shared secret"] # [doc = " to compute the PRF, writing the result into `output`."] # [doc = ""] # [doc = " The caller guarantees that `label`, `seed` are non-empty. The caller makes no"] # [doc = " guarantees about the contents of `peer_pub_key`. It must be validated by"] # [doc = " [`ActiveKeyExchange::complete`]."] fn for_key_exchange (& self , output : & mut [u8 ; 48] , kx : Box < dyn ActiveKeyExchange > , peer_pub_key : & [u8] , label : & [u8] , seed : & [u8] ,) -> Result < () , Error > ; # [doc = " Returns an object that can compute `PRF(secret, label, seed)` with"] # [doc = " the same `master_secret`."] # [doc = ""] # [doc = " This object can amortize any preprocessing needed on `master_secret` over"] # [doc = " several `PRF(...)` calls."] fn new_secret (& self , master_secret : & [u8 ; 48]) -> Box < dyn PrfSecret > ; # [doc = " Return `true` if this is backed by a FIPS-approved implementation."] fn fips (& self) -> bool { false } }
};
}
