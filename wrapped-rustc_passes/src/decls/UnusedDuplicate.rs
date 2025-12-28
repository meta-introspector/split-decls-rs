macro_rules! UnusedDuplicate {
    () => {
        # [derive (LintDiagnostic)] # [diag (passes_unused_duplicate)] pub (crate) struct UnusedDuplicate { # [suggestion (code = "" , applicability = "machine-applicable")] pub this : Span , # [note] pub other : Span , # [warning] pub warning : bool , }
    };
}

UnusedDuplicate!()