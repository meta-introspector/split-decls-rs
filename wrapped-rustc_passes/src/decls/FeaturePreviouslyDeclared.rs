macro_rules! FeaturePreviouslyDeclared {
    () => {
        # [derive (Diagnostic)] # [diag (passes_feature_previously_declared , code = E0711)] pub (crate) struct FeaturePreviouslyDeclared < 'a > { # [primary_span] pub span : Span , pub feature : Symbol , pub declared : & 'a str , pub prev_declared : & 'a str , }
    };
}

FeaturePreviouslyDeclared!();