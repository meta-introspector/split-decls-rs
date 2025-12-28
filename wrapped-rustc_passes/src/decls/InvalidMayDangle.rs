macro_rules! InvalidMayDangle {
    () => {
        # [derive (Diagnostic)] # [diag (passes_may_dangle)] pub (crate) struct InvalidMayDangle { # [primary_span] pub attr_span : Span , }
    };
}

InvalidMayDangle!();