// Generated macro for impl_138 (impl)
macro_rules! Depcrate_signing_keyimpl_138 {
() => {
// Module: crate::signing_key
// Provides: {"impl_138"}
// Dependencies: {}
impl < P : ParameterSet > SignatureAlgorithmIdentifier for SigningKey < P > { type Params = AnyRef < 'static > ; const SIGNATURE_ALGORITHM_IDENTIFIER : AlgorithmIdentifier < Self :: Params > = Signature :: < P > :: ALGORITHM_IDENTIFIER ; }
};
}
