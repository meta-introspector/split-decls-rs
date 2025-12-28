macro_rules! RenamedFeature {
    () => {
        # [derive (Diagnostic)] # [diag (passes_unknown_feature_alias , code = E0635)] pub (crate) struct RenamedFeature { # [primary_span] pub span : Span , pub feature : Symbol , pub alias : Symbol , }
    };
}

RenamedFeature!()