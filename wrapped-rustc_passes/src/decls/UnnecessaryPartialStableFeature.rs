macro_rules! UnnecessaryPartialStableFeature {
    () => {
        # [derive (LintDiagnostic)] # [diag (passes_unnecessary_partial_stable_feature)] pub (crate) struct UnnecessaryPartialStableFeature { # [suggestion (code = "{implies}" , applicability = "maybe-incorrect")] pub span : Span , # [suggestion (passes_suggestion_remove , code = "" , applicability = "maybe-incorrect")] pub line : Span , pub feature : Symbol , pub since : Symbol , pub implies : Symbol , }
    };
}

UnnecessaryPartialStableFeature!()