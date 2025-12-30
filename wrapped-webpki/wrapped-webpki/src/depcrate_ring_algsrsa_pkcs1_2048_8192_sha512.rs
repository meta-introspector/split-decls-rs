// Generated macro for RSA_PKCS1_2048_8192_SHA512 (static)
macro_rules! Depcrate_ring_algsRSA_PKCS1_2048_8192_SHA512 {
() => {
// Module: crate::ring_algs
// Provides: {"RSA_PKCS1_2048_8192_SHA512"}
// Dependencies: {}
# [doc = " RSA PKCS#1 1.5 signatures using SHA-512 for keys of 2048-8192 bits."] # [cfg (feature = "alloc")] pub static RSA_PKCS1_2048_8192_SHA512 : & dyn SignatureVerificationAlgorithm = & RingAlgorithm { public_key_alg_id : alg_id :: RSA_ENCRYPTION , signature_alg_id : alg_id :: RSA_PKCS1_SHA512 , verification_alg : & signature :: RSA_PKCS1_2048_8192_SHA512 , } ;
};
}
