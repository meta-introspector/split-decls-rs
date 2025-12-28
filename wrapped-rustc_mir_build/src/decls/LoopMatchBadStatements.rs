macro_rules! LoopMatchBadStatements {
    () => {
        # [derive (Diagnostic)] # [diag (mir_build_loop_match_bad_statements)] pub (crate) struct LoopMatchBadStatements { # [primary_span] pub span : Span , }
    };
}

LoopMatchBadStatements!();