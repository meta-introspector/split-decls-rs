// Generated macro for PrfSecret (trait)
macro_rules! Depcrate_crypto_tls12PrfSecret {
() => {
// Module: crate::crypto::tls12
// Provides: {"PrfSecret"}
// Dependencies: {}
# [doc = " An instantiation of the TLS1.2 PRF with a fixed hash function and master secret."] pub trait PrfSecret : Send + Sync { # [doc = " Computes `PRF(secret, label, seed)`, writing the result into `output`."] # [doc = ""] # [doc = " `secret` is implicit in this object; see [`Prf::new_secret`]."] # [doc = ""] # [doc = " The caller guarantees that `label` and `seed` are non-empty."] fn prf (& self , output : & mut [u8] , label : & [u8] , seed : & [u8]) ; }
};
}
