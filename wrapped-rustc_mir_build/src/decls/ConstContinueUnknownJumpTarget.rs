macro_rules! ConstContinueUnknownJumpTarget {
    () => {
        # [derive (Diagnostic)] # [diag (mir_build_const_continue_unknown_jump_target)] pub (crate) struct ConstContinueUnknownJumpTarget { # [primary_span] pub span : Span , }
    };
}

ConstContinueUnknownJumpTarget!();