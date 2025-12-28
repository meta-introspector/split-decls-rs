macro_rules! UnrecognizedArgument {
    () => {
        # [derive (Diagnostic)] # [diag (passes_unrecognized_argument)] pub (crate) struct UnrecognizedArgument { # [primary_span] pub span : Span , }
    };
}

UnrecognizedArgument!()