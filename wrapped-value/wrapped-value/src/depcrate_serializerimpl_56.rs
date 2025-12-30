// Generated macro for impl_56 (impl)
macro_rules! Depcrate_serializerimpl_56 {
() => {
// Module: crate::serializer
// Provides: {"impl_56"}
// Dependencies: {}
impl ser :: Error for SerializerError { fn custom < T : fmt :: Display > (msg : T) -> SerializerError { SerializerError (msg . to_string ()) } }
};
}
