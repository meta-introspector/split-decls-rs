// Generated macro for RSA_PKCS1_2048_8192_SHA256 (static)
macro_rules! Depcrate_aws_lc_rs_algsRSA_PKCS1_2048_8192_SHA256 {
() => {
// Module: crate::aws_lc_rs_algs
// Provides: {"RSA_PKCS1_2048_8192_SHA256"}
// Dependencies: {}
# [doc = " RSA PKCS#1 1.5 signatures using SHA-256 for keys of 2048-8192 bits."] pub static RSA_PKCS1_2048_8192_SHA256 : & dyn SignatureVerificationAlgorithm = & AwsLcRsAlgorithm { public_key_alg_id : alg_id :: RSA_ENCRYPTION , signature_alg_id : alg_id :: RSA_PKCS1_SHA256 , verification_alg : & signature :: RSA_PKCS1_2048_8192_SHA256 , in_fips_submission : true , } ;
};
}
