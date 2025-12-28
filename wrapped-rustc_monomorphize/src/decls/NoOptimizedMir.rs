macro_rules! NoOptimizedMir {
    () => {
        # [derive (Diagnostic)] # [diag (monomorphize_no_optimized_mir)] pub (crate) struct NoOptimizedMir { # [note] pub span : Span , pub crate_name : Symbol , pub instance : String , }
    };
}

NoOptimizedMir!();