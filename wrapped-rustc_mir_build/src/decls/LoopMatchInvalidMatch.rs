macro_rules! LoopMatchInvalidMatch {
    () => {
        # [derive (Diagnostic)] # [diag (mir_build_loop_match_invalid_match)] # [note] pub (crate) struct LoopMatchInvalidMatch { # [primary_span] pub span : Span , }
    };
}

LoopMatchInvalidMatch!();