// Generated macro for AbsPathWithModuleSugg (struct)
macro_rules! Depcrate_lintsAbsPathWithModuleSugg {
() => {
// Module: crate::lints
// Provides: {"AbsPathWithModuleSugg"}
// Dependencies: {}
# [derive (Subdiagnostic)] # [suggestion (lint_suggestion , code = "{replacement}")] pub (crate) struct AbsPathWithModuleSugg { # [primary_span] pub span : Span , # [applicability] pub applicability : Applicability , pub replacement : String , }
};
}
