// Generated macro for InvalidPattern (struct)
macro_rules! Depcrate_errorsInvalidPattern {
() => {
// Module: crate::errors
// Provides: {"InvalidPattern"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (mir_build_invalid_pattern)] pub (crate) struct InvalidPattern < 'tcx > { # [primary_span] # [label] pub (crate) span : Span , pub (crate) non_sm_ty : Ty < 'tcx > , pub (crate) prefix : String , }
};
}
