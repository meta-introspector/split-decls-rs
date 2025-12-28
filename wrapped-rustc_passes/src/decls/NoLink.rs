macro_rules! NoLink {
    () => {
        # [derive (Diagnostic)] # [diag (passes_no_link)] pub (crate) struct NoLink { # [primary_span] pub attr_span : Span , # [label] pub span : Span , }
    };
}

NoLink!();