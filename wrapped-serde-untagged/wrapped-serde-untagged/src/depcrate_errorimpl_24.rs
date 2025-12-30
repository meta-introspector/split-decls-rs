// Generated macro for impl_24 (impl)
macro_rules! Depcrate_errorimpl_24 {
() => {
// Module: crate::error
// Provides: {"impl_24"}
// Dependencies: {}
impl serde :: de :: Error for Error { fn custom < T : Display > (msg : T) -> Self { let imp = ErrorImpl :: Custom (msg . to_string ()) ; Error { imp } } fn invalid_type (unexpected : serde :: de :: Unexpected , expected : & dyn Expected) -> Self { let imp = ErrorImpl :: InvalidType { unexpected : Unexpected :: from_serde (unexpected) , expected : expected . to_string () , } ; Error { imp } } fn invalid_value (unexpected : serde :: de :: Unexpected , expected : & dyn Expected) -> Self { let imp = ErrorImpl :: InvalidValue { unexpected : Unexpected :: from_serde (unexpected) , expected : expected . to_string () , } ; Error { imp } } fn invalid_length (len : usize , expected : & dyn Expected) -> Self { let imp = ErrorImpl :: InvalidLength { len , expected : expected . to_string () , } ; Error { imp } } fn unknown_variant (variant : & str , expected : & 'static [& 'static str]) -> Self { let imp = ErrorImpl :: UnknownVariant { variant : variant . to_owned () , expected , } ; Error { imp } } fn unknown_field (field : & str , expected : & 'static [& 'static str]) -> Self { let imp = ErrorImpl :: UnknownField { field : field . to_owned () , expected , } ; Error { imp } } fn missing_field (field : & 'static str) -> Self { let imp = ErrorImpl :: MissingField { field } ; Error { imp } } fn duplicate_field (field : & 'static str) -> Self { let imp = ErrorImpl :: DuplicateField { field } ; Error { imp } } }
};
}
