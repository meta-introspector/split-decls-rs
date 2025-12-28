macro_rules! DiagnosticOnUnimplementedOnlyForTraits {
    () => {
        # [derive (LintDiagnostic)] # [diag (passes_diagnostic_diagnostic_on_unimplemented_only_for_traits)] struct DiagnosticOnUnimplementedOnlyForTraits ;
    };
}

DiagnosticOnUnimplementedOnlyForTraits!();