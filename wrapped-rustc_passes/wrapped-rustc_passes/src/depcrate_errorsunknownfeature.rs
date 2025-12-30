// Generated macro for UnknownFeature (struct)
macro_rules! Depcrate_errorsUnknownFeature {
() => {
// Module: crate::errors
// Provides: {"UnknownFeature"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (passes_unknown_feature , code = E0635)] pub (crate) struct UnknownFeature { # [primary_span] pub span : Span , pub feature : Symbol , }
};
}
