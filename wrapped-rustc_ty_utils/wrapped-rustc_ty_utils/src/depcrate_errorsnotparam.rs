// Generated macro for NotParam (struct)
macro_rules! Depcrate_errorsNotParam {
() => {
// Module: crate::errors
// Provides: {"NotParam"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (ty_utils_impl_trait_not_param , code = E0792)] pub (crate) struct NotParam < 'tcx > { pub arg : GenericArg < 'tcx > , # [primary_span] # [label] pub span : Span , # [note] pub opaque_span : Span , }
};
}
