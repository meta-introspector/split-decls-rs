// Generated macro for impl_831 (impl)
macro_rules! Depcrate_crypto_ring_signimpl_831 {
() => {
// Module: crate::crypto::ring::sign
// Provides: {"impl_831"}
// Dependencies: {}
impl Ed25519Signer { fn sign (& self , message : & [u8]) -> Result < Vec < u8 > , Error > { Ok (self . key . sign (message) . as_ref () . into ()) } }
};
}
