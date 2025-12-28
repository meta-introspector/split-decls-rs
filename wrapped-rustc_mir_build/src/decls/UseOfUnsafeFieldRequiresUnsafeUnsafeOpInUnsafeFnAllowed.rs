macro_rules! deps {
    () => {
        UnsafeNotInheritedNote!();
    };
}

macro_rules! UseOfUnsafeFieldRequiresUnsafeUnsafeOpInUnsafeFnAllowed {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (mir_build_unsafe_field_requires_unsafe_unsafe_op_in_unsafe_fn_allowed , code = E0133)] # [note] pub (crate) struct UseOfUnsafeFieldRequiresUnsafeUnsafeOpInUnsafeFnAllowed { # [primary_span] # [label] pub (crate) span : Span , # [subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }
    };
}

UseOfUnsafeFieldRequiresUnsafeUnsafeOpInUnsafeFnAllowed!()