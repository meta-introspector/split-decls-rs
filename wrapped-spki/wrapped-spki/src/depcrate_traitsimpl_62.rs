// Generated macro for impl_62 (impl)
macro_rules! Depcrate_traitsimpl_62 {
() => {
// Module: crate::traits
// Provides: {"impl_62"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < T > DynAssociatedAlgorithmIdentifier for T where T : AssociatedAlgorithmIdentifier , { fn algorithm_identifier (& self) -> Result < AlgorithmIdentifierOwned > { Ok (AlgorithmIdentifierOwned { oid : T :: ALGORITHM_IDENTIFIER . oid , parameters : T :: ALGORITHM_IDENTIFIER . parameters . as_ref () . map (Any :: encode_from) . transpose () ? , }) } }
};
}
