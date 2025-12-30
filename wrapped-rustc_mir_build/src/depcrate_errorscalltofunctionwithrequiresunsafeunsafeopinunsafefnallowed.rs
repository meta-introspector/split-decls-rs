// Generated macro for CallToFunctionWithRequiresUnsafeUnsafeOpInUnsafeFnAllowed (struct)
macro_rules! Depcrate_errorsCallToFunctionWithRequiresUnsafeUnsafeOpInUnsafeFnAllowed {
() => {
// Module: crate::errors
// Provides: {"CallToFunctionWithRequiresUnsafeUnsafeOpInUnsafeFnAllowed"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (mir_build_call_to_fn_with_requires_unsafe_unsafe_op_in_unsafe_fn_allowed , code = E0133)] # [help] pub (crate) struct CallToFunctionWithRequiresUnsafeUnsafeOpInUnsafeFnAllowed { # [primary_span] # [label] pub (crate) span : Span , pub (crate) function : String , pub (crate) missing_target_features : DiagArgValue , pub (crate) missing_target_features_count : usize , # [note] pub (crate) note : bool , pub (crate) build_target_features : DiagArgValue , pub (crate) build_target_features_count : usize , # [subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }
};
}
