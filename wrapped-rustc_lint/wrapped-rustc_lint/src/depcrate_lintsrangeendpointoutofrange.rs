// Generated macro for RangeEndpointOutOfRange (struct)
macro_rules! Depcrate_lintsRangeEndpointOutOfRange {
() => {
// Module: crate::lints
// Provides: {"RangeEndpointOutOfRange"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_range_endpoint_out_of_range)] pub (crate) struct RangeEndpointOutOfRange < 'a > { pub ty : & 'a str , # [subdiagnostic] pub sub : UseInclusiveRange < 'a > , }
};
}
