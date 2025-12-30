// Generated macro for impl_1151 (impl)
macro_rules! Depcrate_crypto_aws_lc_rs_signimpl_1151 {
() => {
// Module: crate::crypto::aws_lc_rs::sign
// Provides: {"impl_1151"}
// Dependencies: {}
impl RsaSigner { fn sign (& self , message : & [u8]) -> Result < Vec < u8 > , Error > { let mut sig = vec ! [0 ; self . key . public_modulus_len ()] ; let rng = SystemRandom :: new () ; self . key . sign (self . encoding , & rng , message , & mut sig) . map (| _ | sig) . map_err (| _ | Error :: General ("signing failed" . to_string ())) } }
};
}
