macro_rules! deps {
    () => {
        Uncovered!();
    };
}

macro_rules! NonExhaustiveOmittedPattern {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (pattern_analysis_non_exhaustive_omitted_pattern)] # [help] # [note] pub (crate) struct NonExhaustiveOmittedPattern < 'tcx > { pub scrut_ty : Ty < 'tcx > , # [subdiagnostic] pub uncovered : Uncovered , }
    };
}

NonExhaustiveOmittedPattern!()