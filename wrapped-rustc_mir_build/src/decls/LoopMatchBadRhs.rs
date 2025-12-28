macro_rules! LoopMatchBadRhs {
    () => {
        # [derive (Diagnostic)] # [diag (mir_build_loop_match_bad_rhs)] pub (crate) struct LoopMatchBadRhs { # [primary_span] pub span : Span , }
    };
}

LoopMatchBadRhs!();