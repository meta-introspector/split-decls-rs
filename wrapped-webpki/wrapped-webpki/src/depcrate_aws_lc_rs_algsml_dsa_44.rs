// Generated macro for ML_DSA_44 (static)
macro_rules! Depcrate_aws_lc_rs_algsML_DSA_44 {
() => {
// Module: crate::aws_lc_rs_algs
// Provides: {"ML_DSA_44"}
// Dependencies: {}
# [doc = " ML-DSA signatures using the [4, 4] matrix (security strength category 2)."] # [cfg (all (feature = "aws-lc-rs-unstable" , not (feature = "aws-lc-rs-fips")))] pub static ML_DSA_44 : & dyn SignatureVerificationAlgorithm = & AwsLcRsAlgorithm { public_key_alg_id : alg_id :: ML_DSA_44 , signature_alg_id : alg_id :: ML_DSA_44 , verification_alg : & unstable :: signature :: ML_DSA_44 , in_fips_submission : false , } ;
};
}
