// Generated macro for AbiRequiredTargetFeature (struct)
macro_rules! Depcrate_errorsAbiRequiredTargetFeature {
() => {
// Module: crate::errors
// Provides: {"AbiRequiredTargetFeature"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (monomorphize_abi_required_target_feature)] # [help] pub (crate) struct AbiRequiredTargetFeature < 'a > { # [primary_span] # [label] pub span : Span , pub required_feature : & 'a str , pub abi : & 'a str , # [doc = " Whether this is a problem at a call site or at a declaration."] pub is_call : bool , }
};
}
