macro_rules! ConstContinueMissingLabelOrValue {
    () => {
        # [derive (Diagnostic)] # [diag (mir_build_const_continue_missing_label_or_value)] pub (crate) struct ConstContinueMissingLabelOrValue { # [primary_span] pub span : Span , }
    };
}

ConstContinueMissingLabelOrValue!();