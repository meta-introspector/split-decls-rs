macro_rules! LowerRangeBoundMustBeLessThanUpper {
    () => {
        # [derive (Diagnostic)] # [diag (mir_build_lower_range_bound_must_be_less_than_upper , code = E0579)] pub (crate) struct LowerRangeBoundMustBeLessThanUpper { # [primary_span] pub (crate) span : Span , }
    };
}

LowerRangeBoundMustBeLessThanUpper!()