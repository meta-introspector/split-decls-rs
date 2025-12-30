// Generated macro for ED25519 (static)
macro_rules! Depcrate_aws_lc_rs_algsED25519 {
() => {
// Module: crate::aws_lc_rs_algs
// Provides: {"ED25519"}
// Dependencies: {}
# [doc = " ED25519 signatures according to RFC 8410"] pub static ED25519 : & dyn SignatureVerificationAlgorithm = & AwsLcRsAlgorithm { public_key_alg_id : alg_id :: ED25519 , signature_alg_id : alg_id :: ED25519 , verification_alg : & signature :: ED25519 , in_fips_submission : true , } ;
};
}
