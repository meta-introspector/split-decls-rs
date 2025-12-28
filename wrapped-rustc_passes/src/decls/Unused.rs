macro_rules! deps {
    () => {
        UnusedNote!();
    };
}

macro_rules! Unused {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (passes_unused)] pub (crate) struct Unused { # [suggestion (code = "" , applicability = "machine-applicable")] pub attr_span : Span , # [subdiagnostic] pub note : UnusedNote , }
    };
}

Unused!()