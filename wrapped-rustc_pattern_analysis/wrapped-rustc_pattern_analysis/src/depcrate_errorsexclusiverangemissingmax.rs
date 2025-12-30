// Generated macro for ExclusiveRangeMissingMax (struct)
macro_rules! Depcrate_errorsExclusiveRangeMissingMax {
() => {
// Module: crate::errors
// Provides: {"ExclusiveRangeMissingMax"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (pattern_analysis_excluside_range_missing_max)] pub struct ExclusiveRangeMissingMax { # [label] # [suggestion (code = "{suggestion}" , applicability = "maybe-incorrect")] # [doc = " This is an exclusive range that looks like `lo..max` (i.e. doesn't match `max`)."] pub first_range : Span , # [doc = " Suggest `lo..=max` instead."] pub suggestion : String , pub max : String , }
};
}
