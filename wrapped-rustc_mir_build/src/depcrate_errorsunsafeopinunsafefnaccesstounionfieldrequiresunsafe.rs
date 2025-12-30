// Generated macro for UnsafeOpInUnsafeFnAccessToUnionFieldRequiresUnsafe (struct)
macro_rules! Depcrate_errorsUnsafeOpInUnsafeFnAccessToUnionFieldRequiresUnsafe {
() => {
// Module: crate::errors
// Provides: {"UnsafeOpInUnsafeFnAccessToUnionFieldRequiresUnsafe"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (mir_build_unsafe_op_in_unsafe_fn_union_field_requires_unsafe , code = E0133)] # [note] pub (crate) struct UnsafeOpInUnsafeFnAccessToUnionFieldRequiresUnsafe { # [label] pub (crate) span : Span , # [subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedLintNote > , }
};
}
