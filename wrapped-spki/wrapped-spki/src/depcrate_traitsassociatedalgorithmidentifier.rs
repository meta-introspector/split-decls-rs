// Generated macro for AssociatedAlgorithmIdentifier (trait)
macro_rules! Depcrate_traitsAssociatedAlgorithmIdentifier {
() => {
// Module: crate::traits
// Provides: {"AssociatedAlgorithmIdentifier"}
// Dependencies: {}
# [doc = " Returns `AlgorithmIdentifier` associated with the structure."] # [doc = ""] # [doc = " This is useful for e.g. keys for digital signature algorithms."] pub trait AssociatedAlgorithmIdentifier { # [doc = " Algorithm parameters."] type Params : Tagged + EncodeValue ; # [doc = " `AlgorithmIdentifier` for this structure."] const ALGORITHM_IDENTIFIER : AlgorithmIdentifier < Self :: Params > ; }
};
}
