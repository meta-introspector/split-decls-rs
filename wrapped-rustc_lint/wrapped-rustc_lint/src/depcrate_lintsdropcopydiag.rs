// Generated macro for DropCopyDiag (struct)
macro_rules! Depcrate_lintsDropCopyDiag {
() => {
// Module: crate::lints
// Provides: {"DropCopyDiag"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_dropping_copy_types)] pub (crate) struct DropCopyDiag < 'a > { pub arg_ty : Ty < 'a > , # [label] pub label : Span , # [subdiagnostic] pub sugg : UseLetUnderscoreIgnoreSuggestion , }
};
}
