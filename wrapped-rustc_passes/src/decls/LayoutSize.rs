macro_rules! LayoutSize {
    () => {
        # [derive (Diagnostic)] # [diag (passes_layout_size)] pub (crate) struct LayoutSize { # [primary_span] pub span : Span , pub size : String , }
    };
}

LayoutSize!()