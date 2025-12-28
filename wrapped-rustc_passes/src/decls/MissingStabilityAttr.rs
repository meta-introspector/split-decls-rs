macro_rules! MissingStabilityAttr {
    () => {
        # [derive (Diagnostic)] # [diag (passes_missing_stability_attr)] pub (crate) struct MissingStabilityAttr < 'a > { # [primary_span] pub span : Span , pub descr : & 'a str , }
    };
}

MissingStabilityAttr!();