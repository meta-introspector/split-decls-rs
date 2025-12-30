// Generated macro for UnusedExternCrate (struct)
macro_rules! Depcrate_lintsUnusedExternCrate {
() => {
// Module: crate::lints
// Provides: {"UnusedExternCrate"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_unused_extern_crate)] pub (crate) struct UnusedExternCrate { # [label] pub span : Span , # [suggestion (code = "" , applicability = "machine-applicable" , style = "verbose")] pub removal_span : Span , }
};
}
