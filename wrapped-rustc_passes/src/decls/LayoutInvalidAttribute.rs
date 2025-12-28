macro_rules! LayoutInvalidAttribute {
    () => {
        # [derive (Diagnostic)] # [diag (passes_layout_invalid_attribute)] pub (crate) struct LayoutInvalidAttribute { # [primary_span] pub span : Span , }
    };
}

LayoutInvalidAttribute!()