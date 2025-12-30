// Generated macro for impl_1161 (impl)
macro_rules! Depcrate_crypto_aws_lc_rs_signimpl_1161 {
() => {
// Module: crate::crypto::aws_lc_rs::sign
// Provides: {"impl_1161"}
// Dependencies: {}
impl Ed25519Signer { fn sign (& self , message : & [u8]) -> Result < Vec < u8 > , Error > { Ok (self . key . sign (message) . as_ref () . into ()) } }
};
}
