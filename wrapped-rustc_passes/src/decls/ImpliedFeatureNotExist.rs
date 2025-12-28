macro_rules! ImpliedFeatureNotExist {
    () => {
        # [derive (Diagnostic)] # [diag (passes_implied_feature_not_exist)] pub (crate) struct ImpliedFeatureNotExist { # [primary_span] pub span : Span , pub feature : Symbol , pub implied_by : Symbol , }
    };
}

ImpliedFeatureNotExist!();