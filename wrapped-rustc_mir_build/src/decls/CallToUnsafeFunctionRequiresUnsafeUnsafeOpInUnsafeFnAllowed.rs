macro_rules! deps {
    () => {
        UnsafeNotInheritedNote!();
    };
}

macro_rules! CallToUnsafeFunctionRequiresUnsafeUnsafeOpInUnsafeFnAllowed {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (mir_build_call_to_unsafe_fn_requires_unsafe_unsafe_op_in_unsafe_fn_allowed , code = E0133)] # [note] pub (crate) struct CallToUnsafeFunctionRequiresUnsafeUnsafeOpInUnsafeFnAllowed { # [primary_span] # [label] pub (crate) span : Span , pub (crate) function : String , # [subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }
    };
}

CallToUnsafeFunctionRequiresUnsafeUnsafeOpInUnsafeFnAllowed!();