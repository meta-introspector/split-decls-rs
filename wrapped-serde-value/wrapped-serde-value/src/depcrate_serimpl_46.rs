// Generated macro for impl_46 (impl)
macro_rules! Depcrate_serimpl_46 {
() => {
// Module: crate::ser
// Provides: {"impl_46"}
// Dependencies: {}
impl ser :: Error for SerializerError { fn custom < T : fmt :: Display > (msg : T) -> SerializerError { SerializerError :: Custom (msg . to_string ()) } }
};
}
