macro_rules! InnerCrateLevelAttr {
    () => {
        # [derive (LintDiagnostic)] # [diag (passes_inner_crate_level_attr)] pub (crate) struct InnerCrateLevelAttr ;
    };
}

InnerCrateLevelAttr!();