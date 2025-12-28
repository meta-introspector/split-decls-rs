macro_rules! UnusedAssignSuggestion {
    () => {
        # [derive (Subdiagnostic)] # [multipart_suggestion (passes_unused_assign_suggestion , applicability = "maybe-incorrect")] pub (crate) struct UnusedAssignSuggestion { pub pre : & 'static str , # [suggestion_part (code = "{pre}mut ")] pub ty_span : Option < Span > , # [suggestion_part (code = "")] pub ty_ref_span : Span , # [suggestion_part (code = "*")] pub ident_span : Span , # [suggestion_part (code = "")] pub expr_ref_span : Span , }
    };
}

UnusedAssignSuggestion!();