macro_rules! UselessStability {
    () => {
        # [derive (Diagnostic)] # [diag (passes_useless_stability)] pub (crate) struct UselessStability { # [primary_span] # [label] pub span : Span , # [label (passes_item)] pub item_sp : Span , }
    };
}

UselessStability!();