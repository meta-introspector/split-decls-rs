// Generated macro for CannotStabilizeDeprecated (struct)
macro_rules! Depcrate_errorsCannotStabilizeDeprecated {
() => {
// Module: crate::errors
// Provides: {"CannotStabilizeDeprecated"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (passes_cannot_stabilize_deprecated)] pub (crate) struct CannotStabilizeDeprecated { # [primary_span] # [label] pub span : Span , # [label (passes_item)] pub item_sp : Span , }
};
}
