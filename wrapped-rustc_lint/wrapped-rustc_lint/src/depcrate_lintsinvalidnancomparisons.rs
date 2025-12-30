// Generated macro for InvalidNanComparisons (enum)
macro_rules! Depcrate_lintsInvalidNanComparisons {
() => {
// Module: crate::lints
// Provides: {"InvalidNanComparisons"}
// Dependencies: {}
# [derive (LintDiagnostic)] pub (crate) enum InvalidNanComparisons { # [diag (lint_invalid_nan_comparisons_eq_ne)] EqNe { # [subdiagnostic] suggestion : InvalidNanComparisonsSuggestion , } , # [diag (lint_invalid_nan_comparisons_lt_le_gt_ge)] LtLeGtGe , }
};
}
