macro_rules! LayoutAlign {
    () => {
        # [derive (Diagnostic)] # [diag (passes_layout_align)] pub (crate) struct LayoutAlign { # [primary_span] pub span : Span , pub align : String , }
    };
}

LayoutAlign!();