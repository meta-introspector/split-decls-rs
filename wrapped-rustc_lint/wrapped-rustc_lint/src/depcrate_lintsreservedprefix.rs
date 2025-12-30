// Generated macro for ReservedPrefix (struct)
macro_rules! Depcrate_lintsReservedPrefix {
() => {
// Module: crate::lints
// Provides: {"ReservedPrefix"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_reserved_prefix)] pub (crate) struct ReservedPrefix { # [label] pub label : Span , # [suggestion (code = " " , applicability = "machine-applicable")] pub suggestion : Span , pub prefix : String , }
};
}
