macro_rules! deps {
    () => {
        IrrefutableLetPatternsWhileLet!();
        IrrefutableLetPatternsIfLetGuard!();
        LetSource!();
        IrrefutableLetPatternsIfLet!();
        IrrefutableLetPatternsLetElse!();
    };
}

macro_rules! report_irrefutable_let_patterns {
    () => {
        deps!();
        fn report_irrefutable_let_patterns (tcx : TyCtxt < '_ > , id : HirId , source : LetSource , count : usize , span : Span ,) { macro_rules ! emit_diag { ($ lint : tt) => { { tcx . emit_node_span_lint (IRREFUTABLE_LET_PATTERNS , id , span , $ lint { count }) ; } } ; } match source { LetSource :: None | LetSource :: PlainLet | LetSource :: Else => bug ! () , LetSource :: IfLet | LetSource :: ElseIfLet => emit_diag ! (IrrefutableLetPatternsIfLet) , LetSource :: IfLetGuard => emit_diag ! (IrrefutableLetPatternsIfLetGuard) , LetSource :: LetElse => emit_diag ! (IrrefutableLetPatternsLetElse) , LetSource :: WhileLet => emit_diag ! (IrrefutableLetPatternsWhileLet) , } }
    };
}

report_irrefutable_let_patterns!()