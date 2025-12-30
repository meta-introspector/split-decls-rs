// Generated macro for MissingUnsafeOnExtern (struct)
macro_rules! Depcrate_lintsMissingUnsafeOnExtern {
() => {
// Module: crate::lints
// Provides: {"MissingUnsafeOnExtern"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_missing_unsafe_on_extern)] pub (crate) struct MissingUnsafeOnExtern { # [suggestion (code = "unsafe " , applicability = "machine-applicable")] pub suggestion : Span , }
};
}
