macro_rules! UnusedVarRemoveFieldSugg {
    () => {
        # [derive (Subdiagnostic)] # [multipart_suggestion (passes_unused_var_remove_field_suggestion , applicability = "machine-applicable")] pub (crate) struct UnusedVarRemoveFieldSugg { # [suggestion_part (code = "")] pub spans : Vec < Span > , }
    };
}

UnusedVarRemoveFieldSugg!();