macro_rules! CannotStabilizeDeprecated {
    () => {
        # [derive (Diagnostic)] # [diag (passes_cannot_stabilize_deprecated)] pub (crate) struct CannotStabilizeDeprecated { # [primary_span] # [label] pub span : Span , # [label (passes_item)] pub item_sp : Span , }
    };
}

CannotStabilizeDeprecated!()