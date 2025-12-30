// Generated macro for RedundantSemicolonsSuggestion (struct)
macro_rules! Depcrate_lintsRedundantSemicolonsSuggestion {
() => {
// Module: crate::lints
// Provides: {"RedundantSemicolonsSuggestion"}
// Dependencies: {}
# [derive (Subdiagnostic)] # [suggestion (lint_redundant_semicolons_suggestion , code = "" , applicability = "maybe-incorrect")] pub (crate) struct RedundantSemicolonsSuggestion { pub multiple_semicolons : bool , # [primary_span] pub span : Span , }
};
}
