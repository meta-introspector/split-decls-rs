// Generated macro for RSA_PKCS1_3072_8192_SHA384 (static)
macro_rules! Depcrate_aws_lc_rs_algsRSA_PKCS1_3072_8192_SHA384 {
() => {
// Module: crate::aws_lc_rs_algs
// Provides: {"RSA_PKCS1_3072_8192_SHA384"}
// Dependencies: {}
# [doc = " RSA PKCS#1 1.5 signatures using SHA-384 for keys of 3072-8192 bits."] pub static RSA_PKCS1_3072_8192_SHA384 : & dyn SignatureVerificationAlgorithm = & AwsLcRsAlgorithm { public_key_alg_id : alg_id :: RSA_ENCRYPTION , signature_alg_id : alg_id :: RSA_PKCS1_SHA384 , verification_alg : & signature :: RSA_PKCS1_3072_8192_SHA384 , in_fips_submission : true , } ;
};
}
