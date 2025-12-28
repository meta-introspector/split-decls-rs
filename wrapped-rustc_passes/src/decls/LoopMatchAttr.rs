macro_rules! LoopMatchAttr {
    () => {
        # [derive (Diagnostic)] # [diag (passes_loop_match_attr)] pub (crate) struct LoopMatchAttr { # [primary_span] pub attr_span : Span , # [label] pub node_span : Span , }
    };
}

LoopMatchAttr!();