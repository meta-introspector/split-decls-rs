// Generated macro for impl_25 (impl)
macro_rules! Depcrate_errorimpl_25 {
() => {
// Module: crate::error
// Provides: {"impl_25"}
// Dependencies: {}
impl Error { fn as_serde < E : serde :: de :: Error > (& self) -> E { match & self . imp { ErrorImpl :: Custom (msg) => E :: custom (msg) , ErrorImpl :: InvalidType { unexpected , expected , } => E :: invalid_type (unexpected . as_serde () , & expected . as_str ()) , ErrorImpl :: InvalidValue { unexpected , expected , } => E :: invalid_value (unexpected . as_serde () , & expected . as_str ()) , ErrorImpl :: InvalidLength { len , expected } => { E :: invalid_length (* len , & expected . as_str ()) } ErrorImpl :: UnknownVariant { variant , expected } => { E :: unknown_variant (variant , expected) } ErrorImpl :: UnknownField { field , expected } => E :: unknown_field (field , expected) , ErrorImpl :: MissingField { field } => E :: missing_field (field) , ErrorImpl :: DuplicateField { field } => E :: duplicate_field (field) , } } }
};
}
