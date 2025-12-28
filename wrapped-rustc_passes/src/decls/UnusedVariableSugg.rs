macro_rules! UnusedVariableSugg {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum UnusedVariableSugg { # [multipart_suggestion (passes_suggestion , applicability = "maybe-incorrect")] TryPrefixSugg { # [suggestion_part (code = "_{name}")] spans : Vec < Span > , name : String , } , # [help (passes_unused_variable_args_in_macro)] NoSugg { # [primary_span] span : Span , name : String , } , }
    };
}

UnusedVariableSugg!()