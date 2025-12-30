// Generated macro for DOT (static)
macro_rules! Depcrate_scan_exprDOT {
() => {
// Module: crate::scan_expr
// Provides: {"DOT"}
// Dependencies: {}
static DOT : [(Input , Action) ; 3] = [(Keyword ("await") , SetState (& POSTFIX)) , (ConsumeIdent , SetState (& METHOD)) , (ConsumeLiteral , SetState (& POSTFIX)) ,] ;
};
}
