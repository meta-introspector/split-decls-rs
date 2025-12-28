macro_rules! deps {
    () => {
        UnsafeNotInheritedLintNote!();
    };
}

macro_rules! UnsafeOpInUnsafeFnCallToUnsafeFunctionRequiresUnsafe {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (mir_build_unsafe_op_in_unsafe_fn_call_to_unsafe_fn_requires_unsafe , code = E0133)] # [note] pub (crate) struct UnsafeOpInUnsafeFnCallToUnsafeFunctionRequiresUnsafe { # [label] pub (crate) span : Span , pub (crate) function : String , # [subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedLintNote > , }
    };
}

UnsafeOpInUnsafeFnCallToUnsafeFunctionRequiresUnsafe!();