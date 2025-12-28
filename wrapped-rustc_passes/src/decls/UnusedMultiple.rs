macro_rules! UnusedMultiple {
    () => {
        # [derive (Diagnostic)] # [diag (passes_unused_multiple)] pub (crate) struct UnusedMultiple { # [primary_span] # [suggestion (code = "" , applicability = "machine-applicable")] pub this : Span , # [note] pub other : Span , pub name : Symbol , }
    };
}

UnusedMultiple!()