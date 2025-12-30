// Generated macro for BreakWithLabelAndLoopSub (struct)
macro_rules! Depcrate_lintsBreakWithLabelAndLoopSub {
() => {
// Module: crate::lints
// Provides: {"BreakWithLabelAndLoopSub"}
// Dependencies: {}
# [derive (Subdiagnostic)] # [multipart_suggestion (lint_suggestion , applicability = "machine-applicable")] pub (crate) struct BreakWithLabelAndLoopSub { # [suggestion_part (code = "(")] pub left : Span , # [suggestion_part (code = ")")] pub right : Span , }
};
}
