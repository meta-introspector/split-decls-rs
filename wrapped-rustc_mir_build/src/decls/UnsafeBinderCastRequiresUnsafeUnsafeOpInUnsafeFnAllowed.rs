macro_rules! deps {
    () => {
        UnsafeNotInheritedNote!();
    };
}

macro_rules! UnsafeBinderCastRequiresUnsafeUnsafeOpInUnsafeFnAllowed {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (mir_build_unsafe_binder_cast_requires_unsafe_unsafe_op_in_unsafe_fn_allowed , code = E0133 ,)] pub (crate) struct UnsafeBinderCastRequiresUnsafeUnsafeOpInUnsafeFnAllowed { # [primary_span] # [label] pub (crate) span : Span , # [subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }
    };
}

UnsafeBinderCastRequiresUnsafeUnsafeOpInUnsafeFnAllowed!();