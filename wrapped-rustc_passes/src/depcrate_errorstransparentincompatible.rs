// Generated macro for TransparentIncompatible (struct)
macro_rules! Depcrate_errorsTransparentIncompatible {
() => {
// Module: crate::errors
// Provides: {"TransparentIncompatible"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (passes_transparent_incompatible , code = E0692)] pub (crate) struct TransparentIncompatible { # [primary_span] pub hint_spans : Vec < Span > , pub target : String , }
};
}
