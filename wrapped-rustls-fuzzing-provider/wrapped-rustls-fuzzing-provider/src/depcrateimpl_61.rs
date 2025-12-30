// Generated macro for impl_61 (impl)
macro_rules! Depcrateimpl_61 {
() => {
// Module: crate
// Provides: {"impl_61"}
// Dependencies: {}
impl SignatureVerificationAlgorithm for VerifyAlgorithm { fn public_key_alg_id (& self) -> AlgorithmIdentifier { alg_id :: ECDSA_P256 } fn signature_alg_id (& self) -> AlgorithmIdentifier { alg_id :: ECDSA_SHA256 } fn verify_signature (& self , _public_key : & [u8] , _message : & [u8] , signature : & [u8] ,) -> Result < () , InvalidSignature > { match signature { SIGNATURE => Ok (()) , _ => Err (InvalidSignature) , } } }
};
}
