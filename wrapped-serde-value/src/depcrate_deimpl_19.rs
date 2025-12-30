// Generated macro for impl_19 (impl)
macro_rules! Depcrate_deimpl_19 {
() => {
// Module: crate::de
// Provides: {"impl_19"}
// Dependencies: {}
impl de :: Error for DeserializerError { fn custom < T : fmt :: Display > (msg : T) -> Self { DeserializerError :: Custom (msg . to_string ()) } fn invalid_type (unexp : de :: Unexpected , exp : & dyn de :: Expected) -> Self { DeserializerError :: InvalidType (unexp . into () , exp . to_string ()) } fn invalid_value (unexp : de :: Unexpected , exp : & dyn de :: Expected) -> Self { DeserializerError :: InvalidValue (unexp . into () , exp . to_string ()) } fn invalid_length (len : usize , exp : & dyn de :: Expected) -> Self { DeserializerError :: InvalidLength (len , exp . to_string ()) } fn unknown_variant (field : & str , expected : & 'static [& 'static str]) -> Self { DeserializerError :: UnknownVariant (field . into () , expected) } fn unknown_field (field : & str , expected : & 'static [& 'static str]) -> Self { DeserializerError :: UnknownField (field . into () , expected) } fn missing_field (field : & 'static str) -> Self { DeserializerError :: MissingField (field) } fn duplicate_field (field : & 'static str) -> Self { DeserializerError :: DuplicateField (field) } }
};
}
