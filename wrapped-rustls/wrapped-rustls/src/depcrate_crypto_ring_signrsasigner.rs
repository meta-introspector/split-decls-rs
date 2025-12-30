// Generated macro for RsaSigner (struct)
macro_rules! Depcrate_crypto_ring_signRsaSigner {
() => {
// Module: crate::crypto::ring::sign
// Provides: {"RsaSigner"}
// Dependencies: {}
struct RsaSigner { key : Arc < RsaKeyPair > , scheme : SignatureScheme , encoding : & 'static dyn signature :: RsaEncoding , }
};
}
