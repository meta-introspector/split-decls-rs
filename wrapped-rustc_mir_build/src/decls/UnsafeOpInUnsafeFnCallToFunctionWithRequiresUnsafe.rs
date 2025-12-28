macro_rules! deps {
    () => {
        UnsafeNotInheritedLintNote!();
    };
}

macro_rules! UnsafeOpInUnsafeFnCallToFunctionWithRequiresUnsafe {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (mir_build_unsafe_op_in_unsafe_fn_call_to_fn_with_requires_unsafe , code = E0133)] # [help] pub (crate) struct UnsafeOpInUnsafeFnCallToFunctionWithRequiresUnsafe { # [label] pub (crate) span : Span , pub (crate) function : String , pub (crate) missing_target_features : DiagArgValue , pub (crate) missing_target_features_count : usize , # [note] pub (crate) note : bool , pub (crate) build_target_features : DiagArgValue , pub (crate) build_target_features_count : usize , # [subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedLintNote > , }
    };
}

UnsafeOpInUnsafeFnCallToFunctionWithRequiresUnsafe!();