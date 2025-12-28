macro_rules! FieldIsPrivateLabel {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum FieldIsPrivateLabel { # [label (privacy_field_is_private_is_update_syntax_label)] IsUpdateSyntax { # [primary_span] span : Span , rest_field_names : String , rest_len : usize , } , # [label (privacy_field_is_private_label)] Other { # [primary_span] span : Span , } , }
    };
}

FieldIsPrivateLabel!()