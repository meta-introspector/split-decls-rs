// Generated macro for ML_DSA_65 (static)
macro_rules! Depcrate_aws_lc_rs_algsML_DSA_65 {
() => {
// Module: crate::aws_lc_rs_algs
// Provides: {"ML_DSA_65"}
// Dependencies: {}
# [doc = " ML-DSA signatures using the [6, 5] matrix (security strength category 3)."] # [cfg (all (feature = "aws-lc-rs-unstable" , not (feature = "aws-lc-rs-fips")))] pub static ML_DSA_65 : & dyn SignatureVerificationAlgorithm = & AwsLcRsAlgorithm { public_key_alg_id : alg_id :: ML_DSA_65 , signature_alg_id : alg_id :: ML_DSA_65 , verification_alg : & unstable :: signature :: ML_DSA_65 , in_fips_submission : false , } ;
};
}
