// Generated macro for NonCamelCaseTypeSub (enum)
macro_rules! Depcrate_lintsNonCamelCaseTypeSub {
() => {
// Module: crate::lints
// Provides: {"NonCamelCaseTypeSub"}
// Dependencies: {}
# [derive (Subdiagnostic)] pub (crate) enum NonCamelCaseTypeSub { # [label (lint_label)] Label { # [primary_span] span : Span , } , # [suggestion (lint_suggestion , code = "{replace}" , applicability = "maybe-incorrect")] Suggestion { # [primary_span] span : Span , replace : String , } , }
};
}
