// Generated macro for AbiErrorUnsupportedVectorType (struct)
macro_rules! Depcrate_errorsAbiErrorUnsupportedVectorType {
() => {
// Module: crate::errors
// Provides: {"AbiErrorUnsupportedVectorType"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (monomorphize_abi_error_unsupported_vector_type)] pub (crate) struct AbiErrorUnsupportedVectorType < 'a > { # [primary_span] # [label] pub span : Span , pub ty : Ty < 'a > , # [doc = " Whether this is a problem at a call site or at a declaration."] pub is_call : bool , }
};
}
