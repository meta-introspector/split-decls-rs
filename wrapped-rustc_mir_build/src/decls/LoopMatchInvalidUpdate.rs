macro_rules! LoopMatchInvalidUpdate {
    () => {
        # [derive (Diagnostic)] # [diag (mir_build_loop_match_invalid_update)] pub (crate) struct LoopMatchInvalidUpdate { # [primary_span] pub lhs : Span , # [label] pub scrutinee : Span , }
    };
}

LoopMatchInvalidUpdate!()