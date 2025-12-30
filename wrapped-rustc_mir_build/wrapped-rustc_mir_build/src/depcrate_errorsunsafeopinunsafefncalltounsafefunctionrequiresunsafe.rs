// Generated macro for UnsafeOpInUnsafeFnCallToUnsafeFunctionRequiresUnsafe (struct)
macro_rules! Depcrate_errorsUnsafeOpInUnsafeFnCallToUnsafeFunctionRequiresUnsafe {
() => {
// Module: crate::errors
// Provides: {"UnsafeOpInUnsafeFnCallToUnsafeFunctionRequiresUnsafe"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (mir_build_unsafe_op_in_unsafe_fn_call_to_unsafe_fn_requires_unsafe , code = E0133)] # [note] pub (crate) struct UnsafeOpInUnsafeFnCallToUnsafeFunctionRequiresUnsafe { # [label] pub (crate) span : Span , pub (crate) function : String , # [subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedLintNote > , }
};
}
