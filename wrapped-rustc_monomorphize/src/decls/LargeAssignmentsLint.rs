macro_rules! LargeAssignmentsLint {
    () => {
        # [derive (LintDiagnostic)] # [diag (monomorphize_large_assignments)] # [note] pub (crate) struct LargeAssignmentsLint { # [label] pub span : Span , pub size : u64 , pub limit : u64 , }
    };
}

LargeAssignmentsLint!()