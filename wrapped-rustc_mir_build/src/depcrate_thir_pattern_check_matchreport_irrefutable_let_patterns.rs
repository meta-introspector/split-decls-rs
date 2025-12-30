// Generated macro for report_irrefutable_let_patterns (function)
macro_rules! Depcrate_thir_pattern_check_matchreport_irrefutable_let_patterns {
() => {
// Module: crate::thir::pattern::check_match
// Provides: {"report_irrefutable_let_patterns"}
// Dependencies: {}
fn report_irrefutable_let_patterns (tcx : TyCtxt < '_ > , id : HirId , source : LetSource , count : usize , span : Span ,) { macro_rules ! emit_diag { ($ lint : tt) => { { tcx . emit_node_span_lint (IRREFUTABLE_LET_PATTERNS , id , span , $ lint { count }) ; } } ; } match source { LetSource :: None | LetSource :: PlainLet | LetSource :: Else => bug ! () , LetSource :: IfLet | LetSource :: ElseIfLet => emit_diag ! (IrrefutableLetPatternsIfLet) , LetSource :: IfLetGuard => emit_diag ! (IrrefutableLetPatternsIfLetGuard) , LetSource :: LetElse => emit_diag ! (IrrefutableLetPatternsLetElse) , LetSource :: WhileLet => emit_diag ! (IrrefutableLetPatternsWhileLet) , } }
};
}
