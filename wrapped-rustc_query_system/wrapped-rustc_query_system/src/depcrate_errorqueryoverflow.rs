// Generated macro for QueryOverflow (struct)
macro_rules! Depcrate_errorQueryOverflow {
() => {
// Module: crate::error
// Provides: {"QueryOverflow"}
// Dependencies: {}
# [derive (Diagnostic)] # [help] # [diag (query_system_query_overflow)] pub struct QueryOverflow { # [primary_span] pub span : Span , # [subdiagnostic] pub note : QueryOverflowNote , pub suggested_limit : Limit , pub crate_name : Symbol , }
};
}
