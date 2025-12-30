// Generated macro for UnsafeOpInUnsafeFnUseOfUnsafeFieldRequiresUnsafe (struct)
macro_rules! Depcrate_errorsUnsafeOpInUnsafeFnUseOfUnsafeFieldRequiresUnsafe {
() => {
// Module: crate::errors
// Provides: {"UnsafeOpInUnsafeFnUseOfUnsafeFieldRequiresUnsafe"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (mir_build_unsafe_op_in_unsafe_fn_unsafe_field_requires_unsafe , code = E0133)] # [note] pub (crate) struct UnsafeOpInUnsafeFnUseOfUnsafeFieldRequiresUnsafe { # [label] pub (crate) span : Span , # [subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedLintNote > , }
};
}
