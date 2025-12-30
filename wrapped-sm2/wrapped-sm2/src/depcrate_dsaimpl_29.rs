// Generated macro for impl_29 (impl)
macro_rules! Depcrate_dsaimpl_29 {
() => {
// Module: crate::dsa
// Provides: {"impl_29"}
// Dependencies: {}
# [cfg (all (feature = "alloc" , feature = "pkcs8"))] impl AssociatedAlgorithmIdentifier for Signature { type Params = AnyRef < 'static > ; const ALGORITHM_IDENTIFIER : AlgorithmIdentifierRef < 'static > = AlgorithmIdentifierRef { oid : ObjectIdentifier :: new_unwrap ("1.2.156.10197.1.501") , parameters : None , } ; }
};
}
