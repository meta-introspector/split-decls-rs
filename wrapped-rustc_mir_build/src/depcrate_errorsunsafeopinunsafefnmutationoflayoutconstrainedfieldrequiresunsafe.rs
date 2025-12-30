// Generated macro for UnsafeOpInUnsafeFnMutationOfLayoutConstrainedFieldRequiresUnsafe (struct)
macro_rules! Depcrate_errorsUnsafeOpInUnsafeFnMutationOfLayoutConstrainedFieldRequiresUnsafe {
() => {
// Module: crate::errors
// Provides: {"UnsafeOpInUnsafeFnMutationOfLayoutConstrainedFieldRequiresUnsafe"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (mir_build_unsafe_op_in_unsafe_fn_mutation_of_layout_constrained_field_requires_unsafe , code = E0133)] # [note] pub (crate) struct UnsafeOpInUnsafeFnMutationOfLayoutConstrainedFieldRequiresUnsafe { # [label] pub (crate) span : Span , # [subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedLintNote > , }
};
}
