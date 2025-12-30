// Generated macro for impl_7 (impl)
macro_rules! Depcrate_deserializerimpl_7 {
() => {
// Module: crate::deserializer
// Provides: {"impl_7"}
// Dependencies: {}
impl de :: Error for DeserializerError { # [inline] fn custom < T : fmt :: Display > (msg : T) -> Self { DeserializerError (msg . to_string ()) } }
};
}
