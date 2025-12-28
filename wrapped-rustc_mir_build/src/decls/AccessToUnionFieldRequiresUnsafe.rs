macro_rules! deps {
    () => {
        UnsafeNotInheritedNote!();
    };
}

macro_rules! AccessToUnionFieldRequiresUnsafe {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (mir_build_union_field_requires_unsafe , code = E0133)] # [note] pub (crate) struct AccessToUnionFieldRequiresUnsafe { # [primary_span] # [label] pub (crate) span : Span , # [subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }
    };
}

AccessToUnionFieldRequiresUnsafe!()