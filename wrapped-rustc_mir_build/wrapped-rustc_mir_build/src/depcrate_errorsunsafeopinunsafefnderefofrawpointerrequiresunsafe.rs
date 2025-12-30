// Generated macro for UnsafeOpInUnsafeFnDerefOfRawPointerRequiresUnsafe (struct)
macro_rules! Depcrate_errorsUnsafeOpInUnsafeFnDerefOfRawPointerRequiresUnsafe {
() => {
// Module: crate::errors
// Provides: {"UnsafeOpInUnsafeFnDerefOfRawPointerRequiresUnsafe"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (mir_build_unsafe_op_in_unsafe_fn_deref_raw_pointer_requires_unsafe , code = E0133)] # [note] pub (crate) struct UnsafeOpInUnsafeFnDerefOfRawPointerRequiresUnsafe { # [label] pub (crate) span : Span , # [subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedLintNote > , }
};
}
