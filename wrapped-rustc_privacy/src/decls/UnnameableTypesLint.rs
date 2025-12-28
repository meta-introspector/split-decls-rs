macro_rules! UnnameableTypesLint {
    () => {
        # [derive (LintDiagnostic)] # [diag (privacy_unnameable_types_lint)] pub (crate) struct UnnameableTypesLint < 'a > { # [label] pub span : Span , pub kind : & 'a str , pub descr : DiagArgFromDisplay < 'a > , pub reachable_vis : & 'a str , pub reexported_vis : & 'a str , }
    };
}

UnnameableTypesLint!();