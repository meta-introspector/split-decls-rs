macro_rules! MacroOnlyAttribute {
    () => {
        # [derive (Diagnostic)] # [diag (passes_macro_only_attribute)] pub (crate) struct MacroOnlyAttribute { # [primary_span] pub attr_span : Span , # [label] pub span : Span , }
    };
}

MacroOnlyAttribute!()