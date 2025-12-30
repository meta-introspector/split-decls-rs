// Generated macro for ECDSA_P384_SHA256 (static)
macro_rules! Depcrate_ring_algsECDSA_P384_SHA256 {
() => {
// Module: crate::ring_algs
// Provides: {"ECDSA_P384_SHA256"}
// Dependencies: {}
# [doc = " ECDSA signatures using the P-384 curve and SHA-256. Deprecated."] pub static ECDSA_P384_SHA256 : & dyn SignatureVerificationAlgorithm = & RingAlgorithm { public_key_alg_id : alg_id :: ECDSA_P384 , signature_alg_id : alg_id :: ECDSA_SHA256 , verification_alg : & signature :: ECDSA_P384_SHA256_ASN1 , } ;
};
}
