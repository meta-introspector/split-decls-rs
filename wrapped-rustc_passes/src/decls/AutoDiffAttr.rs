macro_rules! AutoDiffAttr {
    () => {
        # [derive (Diagnostic)] # [diag (passes_autodiff_attr)] pub (crate) struct AutoDiffAttr { # [primary_span] # [label] pub attr_span : Span , }
    };
}

AutoDiffAttr!()