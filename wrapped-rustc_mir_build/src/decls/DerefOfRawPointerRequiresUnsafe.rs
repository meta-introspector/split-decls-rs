macro_rules! deps {
    () => {
        UnsafeNotInheritedNote!();
    };
}

macro_rules! DerefOfRawPointerRequiresUnsafe {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (mir_build_deref_raw_pointer_requires_unsafe , code = E0133)] # [note] pub (crate) struct DerefOfRawPointerRequiresUnsafe { # [primary_span] # [label] pub (crate) span : Span , # [subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }
    };
}

DerefOfRawPointerRequiresUnsafe!()