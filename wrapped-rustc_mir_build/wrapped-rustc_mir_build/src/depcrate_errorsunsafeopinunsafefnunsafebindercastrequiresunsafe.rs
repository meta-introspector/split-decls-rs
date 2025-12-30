// Generated macro for UnsafeOpInUnsafeFnUnsafeBinderCastRequiresUnsafe (struct)
macro_rules! Depcrate_errorsUnsafeOpInUnsafeFnUnsafeBinderCastRequiresUnsafe {
() => {
// Module: crate::errors
// Provides: {"UnsafeOpInUnsafeFnUnsafeBinderCastRequiresUnsafe"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (mir_build_unsafe_binder_cast_requires_unsafe , code = E0133 ,)] pub (crate) struct UnsafeOpInUnsafeFnUnsafeBinderCastRequiresUnsafe { # [label] pub (crate) span : Span , # [subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedLintNote > , }
};
}
