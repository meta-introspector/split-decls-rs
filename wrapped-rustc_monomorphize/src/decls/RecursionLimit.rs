macro_rules! RecursionLimit {
    () => {
        # [derive (Diagnostic)] # [diag (monomorphize_recursion_limit)] pub (crate) struct RecursionLimit < 'tcx > { # [primary_span] pub span : Span , pub instance : Instance < 'tcx > , # [note] pub def_span : Span , pub def_path_str : String , }
    };
}

RecursionLimit!()