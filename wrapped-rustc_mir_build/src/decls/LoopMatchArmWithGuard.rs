macro_rules! LoopMatchArmWithGuard {
    () => {
        # [derive (Diagnostic)] # [diag (mir_build_loop_match_arm_with_guard)] pub (crate) struct LoopMatchArmWithGuard { # [primary_span] pub span : Span , }
    };
}

LoopMatchArmWithGuard!()