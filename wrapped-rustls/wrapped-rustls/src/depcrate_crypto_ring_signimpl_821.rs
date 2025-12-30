// Generated macro for impl_821 (impl)
macro_rules! Depcrate_crypto_ring_signimpl_821 {
() => {
// Module: crate::crypto::ring::sign
// Provides: {"impl_821"}
// Dependencies: {}
impl RsaSigner { fn sign (& self , message : & [u8]) -> Result < Vec < u8 > , Error > { let mut sig = vec ! [0 ; self . key . public () . modulus_len ()] ; let rng = SystemRandom :: new () ; self . key . sign (self . encoding , & rng , message , & mut sig) . map (| _ | sig) . map_err (| _ | Error :: General ("signing failed" . to_string ())) } }
};
}
