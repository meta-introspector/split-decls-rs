// Generated macro for ImpliedFeatureNotExist (struct)
macro_rules! Depcrate_errorsImpliedFeatureNotExist {
() => {
// Module: crate::errors
// Provides: {"ImpliedFeatureNotExist"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (passes_implied_feature_not_exist)] pub (crate) struct ImpliedFeatureNotExist { # [primary_span] pub span : Span , pub feature : Symbol , pub implied_by : Symbol , }
};
}
