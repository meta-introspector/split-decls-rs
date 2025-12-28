macro_rules! PeekArgumentUntracked {
    () => {
        # [derive (Diagnostic)] # [diag (mir_dataflow_peek_argument_untracked)] pub (crate) struct PeekArgumentUntracked { # [primary_span] pub span : Span , }
    };
}

PeekArgumentUntracked!();