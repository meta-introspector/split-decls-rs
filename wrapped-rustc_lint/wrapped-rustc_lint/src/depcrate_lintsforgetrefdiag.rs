// Generated macro for ForgetRefDiag (struct)
macro_rules! Depcrate_lintsForgetRefDiag {
() => {
// Module: crate::lints
// Provides: {"ForgetRefDiag"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_forgetting_references)] pub (crate) struct ForgetRefDiag < 'a > { pub arg_ty : Ty < 'a > , # [label] pub label : Span , # [subdiagnostic] pub sugg : UseLetUnderscoreIgnoreSuggestion , }
};
}
