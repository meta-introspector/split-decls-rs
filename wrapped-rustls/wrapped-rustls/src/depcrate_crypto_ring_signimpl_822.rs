// Generated macro for impl_822 (impl)
macro_rules! Depcrate_crypto_ring_signimpl_822 {
() => {
// Module: crate::crypto::ring::sign
// Provides: {"impl_822"}
// Dependencies: {}
impl Signer for RsaSigner { fn sign (self : Box < Self > , message : & [u8]) -> Result < Vec < u8 > , Error > { (* self) . sign (message) } fn scheme (& self) -> SignatureScheme { self . scheme } }
};
}
