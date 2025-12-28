macro_rules! LoopMatchMissingAssignment {
    () => {
        # [derive (Diagnostic)] # [diag (mir_build_loop_match_missing_assignment)] pub (crate) struct LoopMatchMissingAssignment { # [primary_span] pub span : Span , }
    };
}

LoopMatchMissingAssignment!()