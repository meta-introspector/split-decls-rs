macro_rules! WantedConstant {
    () => {
        # [derive (Subdiagnostic)] # [suggestion (mir_build_unreachable_pattern_wanted_const , code = "{const_path}" , applicability = "machine-applicable")] pub (crate) struct WantedConstant { # [primary_span] pub (crate) span : Span , pub (crate) is_typo : bool , pub (crate) const_name : String , pub (crate) const_path : String , }
    };
}

WantedConstant!();