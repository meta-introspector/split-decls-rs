macro_rules! LayoutAbi {
    () => {
        # [derive (Diagnostic)] # [diag (passes_layout_abi)] pub (crate) struct LayoutAbi { # [primary_span] pub span : Span , pub abi : String , }
    };
}

LayoutAbi!();