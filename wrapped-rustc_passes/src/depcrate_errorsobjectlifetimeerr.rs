// Generated macro for ObjectLifetimeErr (struct)
macro_rules! Depcrate_errorsObjectLifetimeErr {
() => {
// Module: crate::errors
// Provides: {"ObjectLifetimeErr"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (passes_object_lifetime_err)] pub (crate) struct ObjectLifetimeErr { # [primary_span] pub span : Span , pub repr : String , }
};
}
