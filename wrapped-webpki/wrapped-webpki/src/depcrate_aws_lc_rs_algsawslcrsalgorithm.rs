// Generated macro for AwsLcRsAlgorithm (struct)
macro_rules! Depcrate_aws_lc_rs_algsAwsLcRsAlgorithm {
() => {
// Module: crate::aws_lc_rs_algs
// Provides: {"AwsLcRsAlgorithm"}
// Dependencies: {}
# [doc = " A `SignatureVerificationAlgorithm` implemented using aws-lc-rs."] # [derive (Debug)] struct AwsLcRsAlgorithm { public_key_alg_id : AlgorithmIdentifier , signature_alg_id : AlgorithmIdentifier , verification_alg : & 'static dyn signature :: VerificationAlgorithm , in_fips_submission : bool , }
};
}
