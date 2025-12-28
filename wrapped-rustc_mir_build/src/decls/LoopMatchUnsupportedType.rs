macro_rules! LoopMatchUnsupportedType {
    () => {
        # [derive (Diagnostic)] # [diag (mir_build_loop_match_unsupported_type)] # [note] pub (crate) struct LoopMatchUnsupportedType < 'tcx > { # [primary_span] pub span : Span , pub ty : Ty < 'tcx > , }
    };
}

LoopMatchUnsupportedType!();