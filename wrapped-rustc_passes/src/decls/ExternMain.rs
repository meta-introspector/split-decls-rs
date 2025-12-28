macro_rules! ExternMain {
    () => {
        # [derive (Diagnostic)] # [diag (passes_extern_main)] pub (crate) struct ExternMain { # [primary_span] pub span : Span , }
    };
}

ExternMain!();