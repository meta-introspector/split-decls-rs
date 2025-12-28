macro_rules! deps {
    () => {
        AttrCrateLevelOnlySugg!();
    };
}

macro_rules! AttrCrateLevelOnly {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (passes_attr_crate_level)] # [note] pub (crate) struct AttrCrateLevelOnly { # [subdiagnostic] pub sugg : Option < AttrCrateLevelOnlySugg > , }
    };
}

AttrCrateLevelOnly!()