// Generated macro for BorrowOfLayoutConstrainedFieldRequiresUnsafe (struct)
macro_rules! Depcrate_errorsBorrowOfLayoutConstrainedFieldRequiresUnsafe {
() => {
// Module: crate::errors
// Provides: {"BorrowOfLayoutConstrainedFieldRequiresUnsafe"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (mir_build_borrow_of_layout_constrained_field_requires_unsafe , code = E0133)] # [note] pub (crate) struct BorrowOfLayoutConstrainedFieldRequiresUnsafe { # [primary_span] # [label] pub (crate) span : Span , # [subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }
};
}
