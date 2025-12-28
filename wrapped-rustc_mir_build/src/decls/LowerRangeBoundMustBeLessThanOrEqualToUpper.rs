macro_rules! LowerRangeBoundMustBeLessThanOrEqualToUpper {
    () => {
        # [derive (Diagnostic)] # [diag (mir_build_lower_range_bound_must_be_less_than_or_equal_to_upper , code = E0030)] pub (crate) struct LowerRangeBoundMustBeLessThanOrEqualToUpper { # [primary_span] # [label] pub (crate) span : Span , # [note (mir_build_teach_note)] pub (crate) teach : bool , }
    };
}

LowerRangeBoundMustBeLessThanOrEqualToUpper!();