// Generated macro for ECDSA_P256_SHA256 (static)
macro_rules! Depcrate_ring_algsECDSA_P256_SHA256 {
() => {
// Module: crate::ring_algs
// Provides: {"ECDSA_P256_SHA256"}
// Dependencies: {}
# [doc = " ECDSA signatures using the P-256 curve and SHA-256."] pub static ECDSA_P256_SHA256 : & dyn SignatureVerificationAlgorithm = & RingAlgorithm { public_key_alg_id : alg_id :: ECDSA_P256 , signature_alg_id : alg_id :: ECDSA_SHA256 , verification_alg : & signature :: ECDSA_P256_SHA256_ASN1 , } ;
};
}
