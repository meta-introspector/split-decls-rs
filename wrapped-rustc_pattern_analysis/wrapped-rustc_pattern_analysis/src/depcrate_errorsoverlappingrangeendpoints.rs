// Generated macro for OverlappingRangeEndpoints (struct)
macro_rules! Depcrate_errorsOverlappingRangeEndpoints {
() => {
// Module: crate::errors
// Provides: {"OverlappingRangeEndpoints"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (pattern_analysis_overlapping_range_endpoints)] # [note] pub struct OverlappingRangeEndpoints { # [label] pub range : Span , # [subdiagnostic] pub overlap : Vec < Overlap > , }
};
}
