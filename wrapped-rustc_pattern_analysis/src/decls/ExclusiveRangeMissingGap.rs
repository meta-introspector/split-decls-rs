macro_rules! deps {
    () => {
        GappedRange!();
    };
}

macro_rules! ExclusiveRangeMissingGap {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (pattern_analysis_excluside_range_missing_gap)] pub struct ExclusiveRangeMissingGap { # [label] # [suggestion (code = "{suggestion}" , applicability = "maybe-incorrect")] # [doc = " This is an exclusive range that looks like `lo..gap` (i.e. doesn't match `gap`)."] pub first_range : Span , pub gap : String , # [doc = " Suggest `lo..=gap` instead."] pub suggestion : String , # [subdiagnostic] # [doc = " All these ranges skipped over `gap` which we think is probably a mistake."] pub gap_with : Vec < GappedRange > , }
    };
}

ExclusiveRangeMissingGap!()