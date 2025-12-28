macro_rules! deps {
    () => {
        UnsafeNotInheritedNote!();
    };
}

macro_rules! BorrowOfLayoutConstrainedFieldRequiresUnsafe {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (mir_build_borrow_of_layout_constrained_field_requires_unsafe , code = E0133)] # [note] pub (crate) struct BorrowOfLayoutConstrainedFieldRequiresUnsafe { # [primary_span] # [label] pub (crate) span : Span , # [subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }
    };
}

BorrowOfLayoutConstrainedFieldRequiresUnsafe!();