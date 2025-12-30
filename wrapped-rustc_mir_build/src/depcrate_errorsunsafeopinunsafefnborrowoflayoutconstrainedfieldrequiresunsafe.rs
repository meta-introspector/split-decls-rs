// Generated macro for UnsafeOpInUnsafeFnBorrowOfLayoutConstrainedFieldRequiresUnsafe (struct)
macro_rules! Depcrate_errorsUnsafeOpInUnsafeFnBorrowOfLayoutConstrainedFieldRequiresUnsafe {
() => {
// Module: crate::errors
// Provides: {"UnsafeOpInUnsafeFnBorrowOfLayoutConstrainedFieldRequiresUnsafe"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (mir_build_unsafe_op_in_unsafe_fn_borrow_of_layout_constrained_field_requires_unsafe , code = E0133 ,)] pub (crate) struct UnsafeOpInUnsafeFnBorrowOfLayoutConstrainedFieldRequiresUnsafe { # [label] pub (crate) span : Span , # [subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedLintNote > , }
};
}
