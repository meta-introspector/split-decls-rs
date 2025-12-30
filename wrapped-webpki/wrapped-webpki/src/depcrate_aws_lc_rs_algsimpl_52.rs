// Generated macro for impl_52 (impl)
macro_rules! Depcrate_aws_lc_rs_algsimpl_52 {
() => {
// Module: crate::aws_lc_rs_algs
// Provides: {"impl_52"}
// Dependencies: {}
impl SignatureVerificationAlgorithm for AwsLcRsAlgorithm { fn public_key_alg_id (& self) -> AlgorithmIdentifier { self . public_key_alg_id } fn signature_alg_id (& self) -> AlgorithmIdentifier { self . signature_alg_id } fn verify_signature (& self , public_key : & [u8] , message : & [u8] , signature : & [u8] ,) -> Result < () , InvalidSignature > { if matches ! (self . public_key_alg_id , alg_id :: ECDSA_P256 | alg_id :: ECDSA_P384 | alg_id :: ECDSA_P521) { match public_key . first () { Some (0x04) | Some (0x02) | Some (0x03) => { } _ => return Err (InvalidSignature) , } ; } signature :: UnparsedPublicKey :: new (self . verification_alg , public_key) . verify (message , signature) . map_err (| _ | InvalidSignature) } fn fips (& self) -> bool { self . in_fips_submission && try_fips_mode () . is_ok () } }
};
}
