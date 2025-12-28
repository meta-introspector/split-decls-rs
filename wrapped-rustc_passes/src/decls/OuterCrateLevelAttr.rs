macro_rules! deps {
    () => {
        OuterCrateLevelAttrSuggestion!();
    };
}

macro_rules! OuterCrateLevelAttr {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (passes_outer_crate_level_attr)] pub (crate) struct OuterCrateLevelAttr { # [subdiagnostic] pub suggestion : OuterCrateLevelAttrSuggestion , }
    };
}

OuterCrateLevelAttr!();