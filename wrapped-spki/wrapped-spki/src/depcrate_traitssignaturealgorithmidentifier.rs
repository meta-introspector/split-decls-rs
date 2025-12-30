// Generated macro for SignatureAlgorithmIdentifier (trait)
macro_rules! Depcrate_traitsSignatureAlgorithmIdentifier {
() => {
// Module: crate::traits
// Provides: {"SignatureAlgorithmIdentifier"}
// Dependencies: {}
# [doc = " Returns `AlgorithmIdentifier` associated with the signature system."] # [doc = ""] # [doc = " Unlike AssociatedAlgorithmIdentifier this is intended to be implemented for public and/or"] # [doc = " private keys."] pub trait SignatureAlgorithmIdentifier { # [doc = " Algorithm parameters."] type Params : Tagged + EncodeValue ; # [doc = " `AlgorithmIdentifier` for the corresponding signature system."] const SIGNATURE_ALGORITHM_IDENTIFIER : AlgorithmIdentifier < Self :: Params > ; }
};
}
