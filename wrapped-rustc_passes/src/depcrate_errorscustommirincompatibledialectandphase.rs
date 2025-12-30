// Generated macro for CustomMirIncompatibleDialectAndPhase (struct)
macro_rules! Depcrate_errorsCustomMirIncompatibleDialectAndPhase {
() => {
// Module: crate::errors
// Provides: {"CustomMirIncompatibleDialectAndPhase"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (passes_custom_mir_incompatible_dialect_and_phase)] pub (crate) struct CustomMirIncompatibleDialectAndPhase { pub dialect : MirDialect , pub phase : MirPhase , # [primary_span] pub attr_span : Span , # [label] pub dialect_span : Span , # [label] pub phase_span : Span , }
};
}
