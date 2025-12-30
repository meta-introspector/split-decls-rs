// Generated macro for InvalidReprAlignForTarget (struct)
macro_rules! Depcrate_errorsInvalidReprAlignForTarget {
() => {
// Module: crate::errors
// Provides: {"InvalidReprAlignForTarget"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (passes_repr_align_greater_than_target_max , code = E0589)] # [note] pub (crate) struct InvalidReprAlignForTarget { # [primary_span] pub span : Span , pub size : u64 , }
};
}
