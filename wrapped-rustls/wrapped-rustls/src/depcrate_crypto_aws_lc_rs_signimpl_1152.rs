// Generated macro for impl_1152 (impl)
macro_rules! Depcrate_crypto_aws_lc_rs_signimpl_1152 {
() => {
// Module: crate::crypto::aws_lc_rs::sign
// Provides: {"impl_1152"}
// Dependencies: {}
impl Signer for RsaSigner { fn sign (self : Box < Self > , message : & [u8]) -> Result < Vec < u8 > , Error > { (* self) . sign (message) } fn scheme (& self) -> SignatureScheme { self . scheme } }
};
}
