macro_rules! MiscPatternSuggestion {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum MiscPatternSuggestion { # [suggestion (mir_build_suggest_attempted_int_lit , code = "_" , applicability = "maybe-incorrect")] AttemptedIntegerLiteral { # [primary_span] start_span : Span , } , }
    };
}

MiscPatternSuggestion!()