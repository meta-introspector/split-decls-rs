macro_rules! RequiresAnArgument {
    () => {
        # [derive (Diagnostic)] # [diag (mir_dataflow_requires_an_argument)] pub (crate) struct RequiresAnArgument { # [primary_span] pub span : Span , pub name : Symbol , }
    };
}

RequiresAnArgument!()