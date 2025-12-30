// Generated macro for UnsafeOpInUnsafeFnCallToUnsafeFunctionRequiresUnsafeNameless (struct)
macro_rules! Depcrate_errorsUnsafeOpInUnsafeFnCallToUnsafeFunctionRequiresUnsafeNameless {
() => {
// Module: crate::errors
// Provides: {"UnsafeOpInUnsafeFnCallToUnsafeFunctionRequiresUnsafeNameless"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (mir_build_unsafe_op_in_unsafe_fn_call_to_unsafe_fn_requires_unsafe_nameless , code = E0133)] # [note] pub (crate) struct UnsafeOpInUnsafeFnCallToUnsafeFunctionRequiresUnsafeNameless { # [label] pub (crate) span : Span , # [subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedLintNote > , }
};
}
