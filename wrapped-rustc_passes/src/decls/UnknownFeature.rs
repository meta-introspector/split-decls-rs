macro_rules! UnknownFeature {
    () => {
        # [derive (Diagnostic)] # [diag (passes_unknown_feature , code = E0635)] pub (crate) struct UnknownFeature { # [primary_span] pub span : Span , pub feature : Symbol , }
    };
}

UnknownFeature!()