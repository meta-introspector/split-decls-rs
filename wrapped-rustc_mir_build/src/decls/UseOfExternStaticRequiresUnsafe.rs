macro_rules! deps {
    () => {
        UnsafeNotInheritedNote!();
    };
}

macro_rules! UseOfExternStaticRequiresUnsafe {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (mir_build_extern_static_requires_unsafe , code = E0133)] # [note] pub (crate) struct UseOfExternStaticRequiresUnsafe { # [primary_span] # [label] pub (crate) span : Span , # [subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }
    };
}

UseOfExternStaticRequiresUnsafe!()