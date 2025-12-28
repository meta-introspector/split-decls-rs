macro_rules! deps {
    () => {
        Overlap!();
    };
}

macro_rules! OverlappingRangeEndpoints {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (pattern_analysis_overlapping_range_endpoints)] # [note] pub struct OverlappingRangeEndpoints { # [label] pub range : Span , # [subdiagnostic] pub overlap : Vec < Overlap > , }
    };
}

OverlappingRangeEndpoints!();