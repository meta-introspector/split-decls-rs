macro_rules! LiteralOutOfRange {
    () => {
        # [derive (Diagnostic)] # [diag (mir_build_literal_in_range_out_of_bounds)] pub (crate) struct LiteralOutOfRange < 'tcx > { # [primary_span] # [label] pub (crate) span : Span , pub (crate) ty : Ty < 'tcx > , pub (crate) min : i128 , pub (crate) max : u128 , }
    };
}

LiteralOutOfRange!();