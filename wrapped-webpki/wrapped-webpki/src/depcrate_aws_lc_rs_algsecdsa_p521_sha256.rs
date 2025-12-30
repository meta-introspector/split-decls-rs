// Generated macro for ECDSA_P521_SHA256 (static)
macro_rules! Depcrate_aws_lc_rs_algsECDSA_P521_SHA256 {
() => {
// Module: crate::aws_lc_rs_algs
// Provides: {"ECDSA_P521_SHA256"}
// Dependencies: {}
# [doc = " ECDSA signatures using the P-521 curve and SHA-256."] pub static ECDSA_P521_SHA256 : & dyn SignatureVerificationAlgorithm = & AwsLcRsAlgorithm { public_key_alg_id : alg_id :: ECDSA_P521 , signature_alg_id : alg_id :: ECDSA_SHA256 , verification_alg : & signature :: ECDSA_P521_SHA256_ASN1 , in_fips_submission : true , } ;
};
}
