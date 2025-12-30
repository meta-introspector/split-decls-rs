// Generated macro for RenamedFeature (struct)
macro_rules! Depcrate_errorsRenamedFeature {
() => {
// Module: crate::errors
// Provides: {"RenamedFeature"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (passes_unknown_feature_alias , code = E0635)] pub (crate) struct RenamedFeature { # [primary_span] pub span : Span , pub feature : Symbol , pub alias : Symbol , }
};
}
