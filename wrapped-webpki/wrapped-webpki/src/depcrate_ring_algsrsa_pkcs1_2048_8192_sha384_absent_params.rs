// Generated macro for RSA_PKCS1_2048_8192_SHA384_ABSENT_PARAMS (static)
macro_rules! Depcrate_ring_algsRSA_PKCS1_2048_8192_SHA384_ABSENT_PARAMS {
() => {
// Module: crate::ring_algs
// Provides: {"RSA_PKCS1_2048_8192_SHA384_ABSENT_PARAMS"}
// Dependencies: {}
# [doc = " RSA PKCS#1 1.5 signatures using SHA-384 for keys of 2048-8192 bits,"] # [doc = " with illegally absent AlgorithmIdentifier parameters."] # [doc = ""] # [doc = " RFC4055 says on sha256WithRSAEncryption and company:"] # [doc = ""] # [doc = " >   When any of these four object identifiers appears within an"] # [doc = " >   AlgorithmIdentifier, the parameters MUST be NULL.  Implementations"] # [doc = " >   MUST accept the parameters being absent as well as present."] # [doc = ""] # [doc = " This algorithm covers the absent case, [`RSA_PKCS1_2048_8192_SHA384`] covers"] # [doc = " the present case."] # [cfg (feature = "alloc")] pub static RSA_PKCS1_2048_8192_SHA384_ABSENT_PARAMS : & dyn SignatureVerificationAlgorithm = & RingAlgorithm { public_key_alg_id : alg_id :: RSA_ENCRYPTION , signature_alg_id : alg_id :: AlgorithmIdentifier :: from_slice (include_bytes ! ("data/alg-rsa-pkcs1-sha384-absent-params.der")) , verification_alg : & signature :: RSA_PKCS1_2048_8192_SHA384 , } ;
};
}
