// Generated macro for ECDSA_P384_SHA384 (static)
macro_rules! Depcrate_aws_lc_rs_algsECDSA_P384_SHA384 {
() => {
// Module: crate::aws_lc_rs_algs
// Provides: {"ECDSA_P384_SHA384"}
// Dependencies: {}
# [doc = " ECDSA signatures using the P-384 curve and SHA-384."] pub static ECDSA_P384_SHA384 : & dyn SignatureVerificationAlgorithm = & AwsLcRsAlgorithm { public_key_alg_id : alg_id :: ECDSA_P384 , signature_alg_id : alg_id :: ECDSA_SHA384 , verification_alg : & signature :: ECDSA_P384_SHA384_ASN1 , in_fips_submission : true , } ;
};
}
