macro_rules! PeekArgumentNotALocal {
    () => {
        # [derive (Diagnostic)] # [diag (mir_dataflow_peek_argument_not_a_local)] pub (crate) struct PeekArgumentNotALocal { # [primary_span] pub span : Span , }
    };
}

PeekArgumentNotALocal!();