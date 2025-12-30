// Generated macro for MissingStabilityAttr (struct)
macro_rules! Depcrate_errorsMissingStabilityAttr {
() => {
// Module: crate::errors
// Provides: {"MissingStabilityAttr"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (passes_missing_stability_attr)] pub (crate) struct MissingStabilityAttr < 'a > { # [primary_span] pub span : Span , pub descr : & 'a str , }
};
}
