macro_rules! deps {
    () => {
        UnsafeNotInheritedLintNote!();
    };
}

macro_rules! UnsafeOpInUnsafeFnUseOfExternStaticRequiresUnsafe {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (mir_build_unsafe_op_in_unsafe_fn_extern_static_requires_unsafe , code = E0133)] # [note] pub (crate) struct UnsafeOpInUnsafeFnUseOfExternStaticRequiresUnsafe { # [label] pub (crate) span : Span , # [subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedLintNote > , }
    };
}

UnsafeOpInUnsafeFnUseOfExternStaticRequiresUnsafe!()