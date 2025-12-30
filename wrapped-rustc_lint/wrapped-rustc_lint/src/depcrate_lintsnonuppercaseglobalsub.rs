// Generated macro for NonUpperCaseGlobalSub (enum)
macro_rules! Depcrate_lintsNonUpperCaseGlobalSub {
() => {
// Module: crate::lints
// Provides: {"NonUpperCaseGlobalSub"}
// Dependencies: {}
# [derive (Subdiagnostic)] pub (crate) enum NonUpperCaseGlobalSub { # [label (lint_label)] Label { # [primary_span] span : Span , } , # [suggestion (lint_suggestion , code = "{replace}")] Suggestion { # [primary_span] span : Span , # [applicability] applicability : Applicability , replace : String , } , }
};
}
