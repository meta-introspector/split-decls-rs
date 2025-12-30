// Generated macro for RingAlgorithm (struct)
macro_rules! Depcrate_ring_algsRingAlgorithm {
() => {
// Module: crate::ring_algs
// Provides: {"RingAlgorithm"}
// Dependencies: {}
# [doc = " A `SignatureVerificationAlgorithm` implemented using *ring*."] # [derive (Debug)] struct RingAlgorithm { public_key_alg_id : AlgorithmIdentifier , signature_alg_id : AlgorithmIdentifier , verification_alg : & 'static dyn signature :: VerificationAlgorithm , }
};
}
