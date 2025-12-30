// Generated macro for impl_129 (impl)
macro_rules! Depcrate_ring_algsimpl_129 {
() => {
// Module: crate::ring_algs
// Provides: {"impl_129"}
// Dependencies: {}
impl SignatureVerificationAlgorithm for RingAlgorithm { fn public_key_alg_id (& self) -> AlgorithmIdentifier { self . public_key_alg_id } fn signature_alg_id (& self) -> AlgorithmIdentifier { self . signature_alg_id } fn verify_signature (& self , public_key : & [u8] , message : & [u8] , signature : & [u8] ,) -> Result < () , InvalidSignature > { signature :: UnparsedPublicKey :: new (self . verification_alg , public_key) . verify (message , signature) . map_err (| _ | InvalidSignature) } }
};
}
