macro_rules! ChangeFields {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum ChangeFields { # [multipart_suggestion (passes_change_fields_to_be_of_unit_type , applicability = "has-placeholders")] ChangeToUnitTypeOrRemove { num : usize , # [suggestion_part (code = "()")] spans : Vec < Span > , } , # [help (passes_remove_fields)] Remove { num : usize } , }
    };
}

ChangeFields!()