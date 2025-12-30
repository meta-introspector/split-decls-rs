// Generated macro for ED25519 (static)
macro_rules! Depcrate_ring_algsED25519 {
() => {
// Module: crate::ring_algs
// Provides: {"ED25519"}
// Dependencies: {}
# [doc = " ED25519 signatures according to RFC 8410"] pub static ED25519 : & dyn SignatureVerificationAlgorithm = & RingAlgorithm { public_key_alg_id : alg_id :: ED25519 , signature_alg_id : alg_id :: ED25519 , verification_alg : & signature :: ED25519 , } ;
};
}
