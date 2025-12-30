// Generated macro for RSA_PSS_2048_8192_SHA256_LEGACY_KEY (static)
macro_rules! Depcrate_aws_lc_rs_algsRSA_PSS_2048_8192_SHA256_LEGACY_KEY {
() => {
// Module: crate::aws_lc_rs_algs
// Provides: {"RSA_PSS_2048_8192_SHA256_LEGACY_KEY"}
// Dependencies: {}
# [doc = " RSA PSS signatures using SHA-256 for keys of 2048-8192 bits and of"] # [doc = " type rsaEncryption; see [RFC 4055 Section 1.2]."] # [doc = ""] # [doc = " [RFC 4055 Section 1.2]: https://tools.ietf.org/html/rfc4055#section-1.2"] pub static RSA_PSS_2048_8192_SHA256_LEGACY_KEY : & dyn SignatureVerificationAlgorithm = & AwsLcRsAlgorithm { public_key_alg_id : alg_id :: RSA_ENCRYPTION , signature_alg_id : alg_id :: RSA_PSS_SHA256 , verification_alg : & signature :: RSA_PSS_2048_8192_SHA256 , in_fips_submission : true , } ;
};
}
