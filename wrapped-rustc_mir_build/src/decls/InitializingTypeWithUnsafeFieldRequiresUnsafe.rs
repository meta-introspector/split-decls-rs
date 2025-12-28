macro_rules! deps {
    () => {
        UnsafeNotInheritedNote!();
    };
}

macro_rules! InitializingTypeWithUnsafeFieldRequiresUnsafe {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (mir_build_initializing_type_with_unsafe_field_requires_unsafe , code = E0133)] # [note] pub (crate) struct InitializingTypeWithUnsafeFieldRequiresUnsafe { # [primary_span] # [label] pub (crate) span : Span , # [subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }
    };
}

InitializingTypeWithUnsafeFieldRequiresUnsafe!()