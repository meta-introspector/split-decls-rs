// Generated macro for ML_DSA_87 (static)
macro_rules! Depcrate_aws_lc_rs_algsML_DSA_87 {
() => {
// Module: crate::aws_lc_rs_algs
// Provides: {"ML_DSA_87"}
// Dependencies: {}
# [doc = " ML-DSA signatures using the [8. 7] matrix (security strength category 5)."] # [cfg (all (feature = "aws-lc-rs-unstable" , not (feature = "aws-lc-rs-fips")))] pub static ML_DSA_87 : & dyn SignatureVerificationAlgorithm = & AwsLcRsAlgorithm { public_key_alg_id : alg_id :: ML_DSA_87 , signature_alg_id : alg_id :: ML_DSA_87 , verification_alg : & unstable :: signature :: ML_DSA_87 , in_fips_submission : false , } ;
};
}
