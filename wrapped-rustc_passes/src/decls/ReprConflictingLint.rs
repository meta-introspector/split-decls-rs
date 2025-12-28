macro_rules! ReprConflictingLint {
    () => {
        # [derive (LintDiagnostic)] # [diag (passes_repr_conflicting , code = E0566)] pub (crate) struct ReprConflictingLint ;
    };
}

ReprConflictingLint!();