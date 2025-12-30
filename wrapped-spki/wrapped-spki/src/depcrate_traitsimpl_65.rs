// Generated macro for impl_65 (impl)
macro_rules! Depcrate_traitsimpl_65 {
() => {
// Module: crate::traits
// Provides: {"impl_65"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < T > DynSignatureAlgorithmIdentifier for T where T : SignatureAlgorithmIdentifier , { fn signature_algorithm_identifier (& self) -> Result < AlgorithmIdentifierOwned > { Ok (AlgorithmIdentifierOwned { oid : T :: SIGNATURE_ALGORITHM_IDENTIFIER . oid , parameters : T :: SIGNATURE_ALGORITHM_IDENTIFIER . parameters . as_ref () . map (Any :: encode_from) . transpose () ? , }) } }
};
}
