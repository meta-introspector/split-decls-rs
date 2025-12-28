macro_rules! DeprecatedAttribute {
    () => {
        # [derive (Diagnostic)] # [diag (passes_deprecated_attribute , code = E0549)] pub (crate) struct DeprecatedAttribute { # [primary_span] pub span : Span , }
    };
}

DeprecatedAttribute!();