// Generated macro for AmbiguousWidePointerComparisonsCastSuggestion (struct)
macro_rules! Depcrate_lintsAmbiguousWidePointerComparisonsCastSuggestion {
() => {
// Module: crate::lints
// Provides: {"AmbiguousWidePointerComparisonsCastSuggestion"}
// Dependencies: {}
# [derive (Subdiagnostic)] # [multipart_suggestion (lint_cast_suggestion , style = "verbose" , applicability = "maybe-incorrect")] pub (crate) struct AmbiguousWidePointerComparisonsCastSuggestion < 'a > { pub (crate) deref_left : & 'a str , pub (crate) deref_right : & 'a str , pub (crate) paren_left : & 'a str , pub (crate) paren_right : & 'a str , pub (crate) l_modifiers : & 'a str , pub (crate) r_modifiers : & 'a str , # [suggestion_part (code = "({deref_left}")] pub (crate) left_before : Option < Span > , # [suggestion_part (code = "{l_modifiers}{paren_left}.cast::<()>()")] pub (crate) left_after : Span , # [suggestion_part (code = "({deref_right}")] pub (crate) right_before : Option < Span > , # [suggestion_part (code = "{r_modifiers}{paren_right}.cast::<()>()")] pub (crate) right_after : Span , }
};
}
