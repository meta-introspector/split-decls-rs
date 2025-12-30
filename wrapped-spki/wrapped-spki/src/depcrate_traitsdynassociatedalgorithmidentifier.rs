// Generated macro for DynAssociatedAlgorithmIdentifier (trait)
macro_rules! Depcrate_traitsDynAssociatedAlgorithmIdentifier {
() => {
// Module: crate::traits
// Provides: {"DynAssociatedAlgorithmIdentifier"}
// Dependencies: {}
# [doc = " Returns `AlgorithmIdentifier` associated with the structure."] # [doc = ""] # [doc = " This is useful for e.g. keys for digital signature algorithms."] # [cfg (feature = "alloc")] pub trait DynAssociatedAlgorithmIdentifier { # [doc = " `AlgorithmIdentifier` for this structure."] fn algorithm_identifier (& self) -> Result < AlgorithmIdentifierOwned > ; }
};
}
