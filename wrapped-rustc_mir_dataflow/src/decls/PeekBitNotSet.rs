macro_rules! PeekBitNotSet {
    () => {
        # [derive (Diagnostic)] # [diag (mir_dataflow_peek_bit_not_set)] pub (crate) struct PeekBitNotSet { # [primary_span] pub span : Span , }
    };
}

PeekBitNotSet!();