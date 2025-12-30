// Generated macro for UnsafeAttrOutsideUnsafeSuggestion (struct)
macro_rules! Depcrate_lintsUnsafeAttrOutsideUnsafeSuggestion {
() => {
// Module: crate::lints
// Provides: {"UnsafeAttrOutsideUnsafeSuggestion"}
// Dependencies: {}
# [derive (Subdiagnostic)] # [multipart_suggestion (lint_unsafe_attr_outside_unsafe_suggestion , applicability = "machine-applicable")] pub (crate) struct UnsafeAttrOutsideUnsafeSuggestion { # [suggestion_part (code = "unsafe(")] pub left : Span , # [suggestion_part (code = ")")] pub right : Span , }
};
}
