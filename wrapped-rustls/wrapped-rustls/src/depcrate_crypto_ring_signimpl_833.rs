// Generated macro for impl_833 (impl)
macro_rules! Depcrate_crypto_ring_signimpl_833 {
() => {
// Module: crate::crypto::ring::sign
// Provides: {"impl_833"}
// Dependencies: {}
impl Signer for Ed25519Signer { fn sign (self : Box < Self > , message : & [u8]) -> Result < Vec < u8 > , Error > { (* self) . sign (message) } fn scheme (& self) -> SignatureScheme { self . scheme } }
};
}
