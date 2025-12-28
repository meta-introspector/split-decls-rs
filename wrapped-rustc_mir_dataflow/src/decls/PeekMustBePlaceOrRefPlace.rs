macro_rules! PeekMustBePlaceOrRefPlace {
    () => {
        # [derive (Diagnostic)] # [diag (mir_dataflow_peek_must_be_place_or_ref_place)] pub (crate) struct PeekMustBePlaceOrRefPlace { # [primary_span] pub span : Span , }
    };
}

PeekMustBePlaceOrRefPlace!();