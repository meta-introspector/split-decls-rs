// Generated macro for UnusedOp (struct)
macro_rules! Depcrate_lintsUnusedOp {
() => {
// Module: crate::lints
// Provides: {"UnusedOp"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_unused_op)] pub (crate) struct UnusedOp < 'a > { pub op : & 'a str , # [label] pub label : Span , # [subdiagnostic] pub suggestion : UnusedOpSuggestion , }
};
}
