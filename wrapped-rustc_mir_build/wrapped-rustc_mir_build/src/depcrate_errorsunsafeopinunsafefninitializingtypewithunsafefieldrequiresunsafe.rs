// Generated macro for UnsafeOpInUnsafeFnInitializingTypeWithUnsafeFieldRequiresUnsafe (struct)
macro_rules! Depcrate_errorsUnsafeOpInUnsafeFnInitializingTypeWithUnsafeFieldRequiresUnsafe {
() => {
// Module: crate::errors
// Provides: {"UnsafeOpInUnsafeFnInitializingTypeWithUnsafeFieldRequiresUnsafe"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (mir_build_unsafe_op_in_unsafe_fn_initializing_type_with_unsafe_field_requires_unsafe , code = E0133)] # [note] pub (crate) struct UnsafeOpInUnsafeFnInitializingTypeWithUnsafeFieldRequiresUnsafe { # [label] pub (crate) span : Span , # [subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedLintNote > , }
};
}
