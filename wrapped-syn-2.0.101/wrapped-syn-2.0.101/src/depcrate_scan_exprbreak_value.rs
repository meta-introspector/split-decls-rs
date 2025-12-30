// Generated macro for BREAK_VALUE (static)
macro_rules! Depcrate_scan_exprBREAK_VALUE {
() => {
// Module: crate::scan_expr
// Provides: {"BREAK_VALUE"}
// Dependencies: {}
static BREAK_VALUE : [(Input , Action) ; 3] = [(ConsumeNestedBrace , SetState (& IF_THEN)) , (CanBeginExpr , SetState (& INIT)) , (Otherwise , SetState (& POSTFIX)) ,] ;
};
}
