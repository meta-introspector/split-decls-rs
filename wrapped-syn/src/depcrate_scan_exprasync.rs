// Generated macro for ASYNC (static)
macro_rules! Depcrate_scan_exprASYNC {
() => {
// Module: crate::scan_expr
// Provides: {"ASYNC"}
// Dependencies: {}
static ASYNC : [(Input , Action) ; 3] = [(Keyword ("move") , SetState (& ASYNC)) , (Punct ("|") , SetState (& CLOSURE_ARGS)) , (ConsumeBrace , SetState (& POSTFIX)) ,] ;
};
}
