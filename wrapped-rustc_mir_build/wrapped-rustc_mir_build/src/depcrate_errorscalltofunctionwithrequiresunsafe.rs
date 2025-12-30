// Generated macro for CallToFunctionWithRequiresUnsafe (struct)
macro_rules! Depcrate_errorsCallToFunctionWithRequiresUnsafe {
() => {
// Module: crate::errors
// Provides: {"CallToFunctionWithRequiresUnsafe"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (mir_build_call_to_fn_with_requires_unsafe , code = E0133)] # [help] pub (crate) struct CallToFunctionWithRequiresUnsafe { # [primary_span] # [label] pub (crate) span : Span , pub (crate) function : String , pub (crate) missing_target_features : DiagArgValue , pub (crate) missing_target_features_count : usize , # [note] pub (crate) note : bool , pub (crate) build_target_features : DiagArgValue , pub (crate) build_target_features_count : usize , # [subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }
};
}
