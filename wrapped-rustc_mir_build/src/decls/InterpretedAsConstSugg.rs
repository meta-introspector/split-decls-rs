macro_rules! InterpretedAsConstSugg {
    () => {
        # [derive (Subdiagnostic)] # [suggestion (mir_build_interpreted_as_const , code = "{variable}_var" , applicability = "maybe-incorrect" , style = "verbose")] pub (crate) struct InterpretedAsConstSugg { # [primary_span] pub (crate) span : Span , pub (crate) variable : String , }
    };
}

InterpretedAsConstSugg!();