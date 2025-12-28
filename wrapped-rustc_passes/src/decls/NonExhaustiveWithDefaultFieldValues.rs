macro_rules! NonExhaustiveWithDefaultFieldValues {
    () => {
        # [derive (Diagnostic)] # [diag (passes_non_exhaustive_with_default_field_values)] pub (crate) struct NonExhaustiveWithDefaultFieldValues { # [primary_span] pub attr_span : Span , # [label] pub defn_span : Span , }
    };
}

NonExhaustiveWithDefaultFieldValues!()