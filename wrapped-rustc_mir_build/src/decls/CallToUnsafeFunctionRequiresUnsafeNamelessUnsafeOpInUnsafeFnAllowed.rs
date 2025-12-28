macro_rules! deps {
    () => {
        UnsafeNotInheritedNote!();
    };
}

macro_rules! CallToUnsafeFunctionRequiresUnsafeNamelessUnsafeOpInUnsafeFnAllowed {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (mir_build_call_to_unsafe_fn_requires_unsafe_nameless_unsafe_op_in_unsafe_fn_allowed , code = E0133)] # [note] pub (crate) struct CallToUnsafeFunctionRequiresUnsafeNamelessUnsafeOpInUnsafeFnAllowed { # [primary_span] # [label] pub (crate) span : Span , # [subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }
    };
}

CallToUnsafeFunctionRequiresUnsafeNamelessUnsafeOpInUnsafeFnAllowed!();