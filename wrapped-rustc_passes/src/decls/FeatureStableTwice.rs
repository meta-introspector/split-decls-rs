macro_rules! FeatureStableTwice {
    () => {
        # [derive (Diagnostic)] # [diag (passes_feature_stable_twice , code = E0711)] pub (crate) struct FeatureStableTwice { # [primary_span] pub span : Span , pub feature : Symbol , pub since : Symbol , pub prev_since : Symbol , }
    };
}

FeatureStableTwice!()