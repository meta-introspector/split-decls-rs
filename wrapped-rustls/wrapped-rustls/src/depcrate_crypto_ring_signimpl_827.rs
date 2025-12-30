// Generated macro for impl_827 (impl)
macro_rules! Depcrate_crypto_ring_signimpl_827 {
() => {
// Module: crate::crypto::ring::sign
// Provides: {"impl_827"}
// Dependencies: {}
impl Signer for EcdsaSigner { fn sign (self : Box < Self > , message : & [u8]) -> Result < Vec < u8 > , Error > { (* self) . sign (message) } fn scheme (& self) -> SignatureScheme { self . scheme } }
};
}
