// Generated macro for impl_88 (impl)
macro_rules! Depcrate_signature_encodingimpl_88 {
() => {
// Module: crate::signature_encoding
// Provides: {"impl_88"}
// Dependencies: {}
impl < P : ParameterSet > AssociatedAlgorithmIdentifier for Signature < P > { type Params = AnyRef < 'static > ; const ALGORITHM_IDENTIFIER : AlgorithmIdentifierRef < 'static > = AlgorithmIdentifierRef { oid : P :: ALGORITHM_OID , parameters : None , } ; }
};
}
