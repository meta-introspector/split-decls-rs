macro_rules! CollapseDebuginfo {
    () => {
        # [derive (Diagnostic)] # [diag (passes_collapse_debuginfo)] pub (crate) struct CollapseDebuginfo { # [primary_span] pub attr_span : Span , # [label] pub defn_span : Span , }
    };
}

CollapseDebuginfo!();