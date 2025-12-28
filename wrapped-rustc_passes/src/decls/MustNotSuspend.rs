macro_rules! MustNotSuspend {
    () => {
        # [derive (Diagnostic)] # [diag (passes_must_not_suspend)] pub (crate) struct MustNotSuspend { # [primary_span] pub attr_span : Span , # [label] pub span : Span , }
    };
}

MustNotSuspend!()