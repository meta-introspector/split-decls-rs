macro_rules! deps {
    () => {
        UnsafeNotInheritedNote!();
    };
}

macro_rules! CallToUnsafeFunctionRequiresUnsafeNameless {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (mir_build_call_to_unsafe_fn_requires_unsafe_nameless , code = E0133)] # [note] pub (crate) struct CallToUnsafeFunctionRequiresUnsafeNameless { # [primary_span] # [label] pub (crate) span : Span , # [subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }
    };
}

CallToUnsafeFunctionRequiresUnsafeNameless!();