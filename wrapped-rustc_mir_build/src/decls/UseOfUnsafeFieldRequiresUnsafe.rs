macro_rules! deps {
    () => {
        UnsafeNotInheritedNote!();
    };
}

macro_rules! UseOfUnsafeFieldRequiresUnsafe {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (mir_build_unsafe_field_requires_unsafe , code = E0133)] # [note] pub (crate) struct UseOfUnsafeFieldRequiresUnsafe { # [primary_span] # [label] pub (crate) span : Span , # [subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }
    };
}

UseOfUnsafeFieldRequiresUnsafe!()