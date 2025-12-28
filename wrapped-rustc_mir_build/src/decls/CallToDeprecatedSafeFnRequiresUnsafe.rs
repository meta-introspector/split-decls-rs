macro_rules! deps {
    () => {
        CallToDeprecatedSafeFnRequiresUnsafeSub!();
    };
}

macro_rules! CallToDeprecatedSafeFnRequiresUnsafe {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (mir_build_call_to_deprecated_safe_fn_requires_unsafe)] pub (crate) struct CallToDeprecatedSafeFnRequiresUnsafe { # [label] pub (crate) span : Span , pub (crate) function : String , pub (crate) guarantee : String , # [subdiagnostic] pub (crate) sub : CallToDeprecatedSafeFnRequiresUnsafeSub , }
    };
}

CallToDeprecatedSafeFnRequiresUnsafe!()