macro_rules! PatternTypo {
    () => {
        # [derive (Subdiagnostic)] # [multipart_suggestion (passes_unused_var_typo , style = "verbose" , applicability = "machine-applicable")] pub (crate) struct PatternTypo { # [suggestion_part (code = "{code}")] pub span : Span , pub code : String , pub item_name : String , pub kind : String , }
    };
}

PatternTypo!();