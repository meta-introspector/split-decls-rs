macro_rules! ReportEffectiveVisibility {
    () => {
        # [derive (Diagnostic)] # [diag (privacy_report_effective_visibility)] pub (crate) struct ReportEffectiveVisibility { # [primary_span] pub span : Span , pub descr : String , }
    };
}

ReportEffectiveVisibility!()