macro_rules! SuggestLet {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum SuggestLet { # [multipart_suggestion (mir_build_suggest_if_let , applicability = "has-placeholders")] If { # [suggestion_part (code = "if ")] start_span : Span , # [suggestion_part (code = " {{ todo!() }}")] semi_span : Span , count : usize , } , # [suggestion (mir_build_suggest_let_else , code = " else {{ todo!() }}" , applicability = "has-placeholders")] Else { # [primary_span] end_span : Span , count : usize , } , }
    };
}

SuggestLet!()