macro_rules! DuplicateFeatureErr {
    () => {
        # [derive (Diagnostic)] # [diag (passes_duplicate_feature_err , code = E0636)] pub (crate) struct DuplicateFeatureErr { # [primary_span] pub span : Span , pub feature : Symbol , }
    };
}

DuplicateFeatureErr!();