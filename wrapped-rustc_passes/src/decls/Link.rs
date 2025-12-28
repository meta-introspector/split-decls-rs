macro_rules! Link {
    () => {
        # [derive (LintDiagnostic)] # [diag (passes_link)] # [warning] pub (crate) struct Link { # [label] pub span : Option < Span > , }
    };
}

Link!()