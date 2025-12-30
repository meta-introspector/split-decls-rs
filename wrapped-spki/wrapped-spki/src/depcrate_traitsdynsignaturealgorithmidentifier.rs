// Generated macro for DynSignatureAlgorithmIdentifier (trait)
macro_rules! Depcrate_traitsDynSignatureAlgorithmIdentifier {
() => {
// Module: crate::traits
// Provides: {"DynSignatureAlgorithmIdentifier"}
// Dependencies: {}
# [doc = " Returns `AlgorithmIdentifier` associated with the signature system."] # [doc = ""] # [doc = " Unlike AssociatedAlgorithmIdentifier this is intended to be implemented for public and/or"] # [doc = " private keys."] # [cfg (feature = "alloc")] pub trait DynSignatureAlgorithmIdentifier { # [doc = " `AlgorithmIdentifier` for the corresponding signature system."] fn signature_algorithm_identifier (& self) -> Result < AlgorithmIdentifierOwned > ; }
};
}
