// Generated macro for DuplicateArg (struct)
macro_rules! Depcrate_errorsDuplicateArg {
() => {
// Module: crate::errors
// Provides: {"DuplicateArg"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (ty_utils_impl_trait_duplicate_arg)] pub (crate) struct DuplicateArg < 'tcx > { pub arg : GenericArg < 'tcx > , # [primary_span] # [label] pub span : Span , # [note] pub opaque_span : Span , }
};
}
