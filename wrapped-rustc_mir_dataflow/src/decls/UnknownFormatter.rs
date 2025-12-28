macro_rules! UnknownFormatter {
    () => {
        # [derive (Diagnostic)] # [diag (mir_dataflow_unknown_formatter)] pub (crate) struct UnknownFormatter { # [primary_span] pub span : Span , }
    };
}

UnknownFormatter!()