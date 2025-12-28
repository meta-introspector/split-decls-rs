macro_rules! AttrCrateLevelOnlySugg {
    () => {
        # [derive (Subdiagnostic)] # [suggestion (passes_suggestion , applicability = "maybe-incorrect" , code = "!" , style = "verbose")] pub (crate) struct AttrCrateLevelOnlySugg { # [primary_span] pub attr : Span , }
    };
}

AttrCrateLevelOnlySugg!();