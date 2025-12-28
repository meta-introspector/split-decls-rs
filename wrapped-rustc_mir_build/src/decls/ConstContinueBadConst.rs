macro_rules! ConstContinueBadConst {
    () => {
        # [derive (Diagnostic)] # [diag (mir_build_const_continue_bad_const)] pub (crate) struct ConstContinueBadConst { # [primary_span] # [label] pub span : Span , }
    };
}

ConstContinueBadConst!();