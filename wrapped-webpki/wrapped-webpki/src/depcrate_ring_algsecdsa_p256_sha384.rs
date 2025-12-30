// Generated macro for ECDSA_P256_SHA384 (static)
macro_rules! Depcrate_ring_algsECDSA_P256_SHA384 {
() => {
// Module: crate::ring_algs
// Provides: {"ECDSA_P256_SHA384"}
// Dependencies: {}
# [doc = " ECDSA signatures using the P-256 curve and SHA-384. Deprecated."] pub static ECDSA_P256_SHA384 : & dyn SignatureVerificationAlgorithm = & RingAlgorithm { public_key_alg_id : alg_id :: ECDSA_P256 , signature_alg_id : alg_id :: ECDSA_SHA384 , verification_alg : & signature :: ECDSA_P256_SHA384_ASN1 , } ;
};
}
