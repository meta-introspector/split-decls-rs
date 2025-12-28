macro_rules! DeprecatedAnnotationHasNoEffect {
    () => {
        # [derive (LintDiagnostic)] # [diag (passes_deprecated_annotation_has_no_effect)] pub (crate) struct DeprecatedAnnotationHasNoEffect { # [suggestion (applicability = "machine-applicable" , code = "")] pub span : Span , }
    };
}

DeprecatedAnnotationHasNoEffect!();