macro_rules! ConstContinueAttr {
    () => {
        # [derive (Diagnostic)] # [diag (passes_const_continue_attr)] pub (crate) struct ConstContinueAttr { # [primary_span] pub attr_span : Span , # [label] pub node_span : Span , }
    };
}

ConstContinueAttr!();