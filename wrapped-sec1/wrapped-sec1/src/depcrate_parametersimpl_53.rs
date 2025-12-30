// Generated macro for impl_53 (impl)
macro_rules! Depcrate_parametersimpl_53 {
() => {
// Module: crate::parameters
// Provides: {"impl_53"}
// Dependencies: {}
impl EncodeValue for EcParameters { fn value_len (& self) -> der :: Result < Length > { match self { Self :: NamedCurve (oid) => oid . value_len () , } } fn encode_value (& self , writer : & mut impl Writer) -> der :: Result < () > { match self { Self :: NamedCurve (oid) => oid . encode_value (writer) , } } }
};
}
