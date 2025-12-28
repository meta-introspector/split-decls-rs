macro_rules! UnnecessaryStableFeature {
    () => {
        # [derive (LintDiagnostic)] # [diag (passes_unnecessary_stable_feature)] pub (crate) struct UnnecessaryStableFeature { pub feature : Symbol , pub since : Symbol , }
    };
}

UnnecessaryStableFeature!();