macro_rules! deps {
    () => {
        UnusedUnsafeEnclosing!();
    };
}

macro_rules! UnusedUnsafe {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (mir_build_unused_unsafe)] pub (crate) struct UnusedUnsafe { # [label] pub (crate) span : Span , # [subdiagnostic] pub (crate) enclosing : Option < UnusedUnsafeEnclosing > , }
    };
}

UnusedUnsafe!();