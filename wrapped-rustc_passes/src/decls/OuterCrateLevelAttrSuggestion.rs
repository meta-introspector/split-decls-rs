macro_rules! OuterCrateLevelAttrSuggestion {
    () => {
        # [derive (Subdiagnostic)] # [multipart_suggestion (passes_outer_crate_level_attr_suggestion , style = "verbose")] pub (crate) struct OuterCrateLevelAttrSuggestion { # [suggestion_part (code = "!")] pub bang_position : Span , }
    };
}

OuterCrateLevelAttrSuggestion!()