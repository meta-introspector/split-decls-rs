macro_rules! PeekMustBeNotTemporary {
    () => {
        # [derive (Diagnostic)] # [diag (mir_dataflow_peek_must_be_not_temporary)] pub (crate) struct PeekMustBeNotTemporary { # [primary_span] pub span : Span , }
    };
}

PeekMustBeNotTemporary!();