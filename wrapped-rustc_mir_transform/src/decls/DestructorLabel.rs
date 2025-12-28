macro_rules! DestructorLabel {
    () => {
        # [derive (Subdiagnostic)] # [note (mir_transform_tail_expr_dtor)] struct DestructorLabel < 'a > { # [primary_span] span : Span , dtor_kind : & 'static str , name : & 'a str , }
    };
}

DestructorLabel!();