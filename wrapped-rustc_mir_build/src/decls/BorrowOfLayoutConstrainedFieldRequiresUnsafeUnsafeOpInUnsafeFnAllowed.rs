macro_rules! deps {
    () => {
        UnsafeNotInheritedNote!();
    };
}

macro_rules! BorrowOfLayoutConstrainedFieldRequiresUnsafeUnsafeOpInUnsafeFnAllowed {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (mir_build_borrow_of_layout_constrained_field_requires_unsafe_unsafe_op_in_unsafe_fn_allowed , code = E0133)] # [note] pub (crate) struct BorrowOfLayoutConstrainedFieldRequiresUnsafeUnsafeOpInUnsafeFnAllowed { # [primary_span] # [label] pub (crate) span : Span , # [subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }
    };
}

BorrowOfLayoutConstrainedFieldRequiresUnsafeUnsafeOpInUnsafeFnAllowed!()