// Generated macro for UnsafeOpInUnsafeFnUseOfInlineAssemblyRequiresUnsafe (struct)
macro_rules! Depcrate_errorsUnsafeOpInUnsafeFnUseOfInlineAssemblyRequiresUnsafe {
() => {
// Module: crate::errors
// Provides: {"UnsafeOpInUnsafeFnUseOfInlineAssemblyRequiresUnsafe"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (mir_build_unsafe_op_in_unsafe_fn_inline_assembly_requires_unsafe , code = E0133)] # [note] pub (crate) struct UnsafeOpInUnsafeFnUseOfInlineAssemblyRequiresUnsafe { # [label] pub (crate) span : Span , # [subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedLintNote > , }
};
}
