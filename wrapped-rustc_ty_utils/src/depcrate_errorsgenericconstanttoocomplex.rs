// Generated macro for GenericConstantTooComplex (struct)
macro_rules! Depcrate_errorsGenericConstantTooComplex {
() => {
// Module: crate::errors
// Provides: {"GenericConstantTooComplex"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (ty_utils_generic_constant_too_complex)] # [help] pub (crate) struct GenericConstantTooComplex { # [primary_span] pub span : Span , # [note (ty_utils_maybe_supported)] pub maybe_supported : bool , # [subdiagnostic] pub sub : GenericConstantTooComplexSub , }
};
}
