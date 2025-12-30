// Generated macro for UnusedDelimSuggestion (struct)
macro_rules! Depcrate_lintsUnusedDelimSuggestion {
() => {
// Module: crate::lints
// Provides: {"UnusedDelimSuggestion"}
// Dependencies: {}
# [derive (Subdiagnostic)] # [multipart_suggestion (lint_suggestion , applicability = "machine-applicable")] pub (crate) struct UnusedDelimSuggestion { # [suggestion_part (code = "{start_replace}")] pub start_span : Span , pub start_replace : & 'static str , # [suggestion_part (code = "{end_replace}")] pub end_span : Span , pub end_replace : & 'static str , }
};
}
