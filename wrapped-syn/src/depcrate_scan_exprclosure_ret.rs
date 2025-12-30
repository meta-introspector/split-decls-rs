// Generated macro for CLOSURE_RET (static)
macro_rules! Depcrate_scan_exprCLOSURE_RET {
() => {
// Module: crate::scan_expr
// Provides: {"CLOSURE_RET"}
// Dependencies: {}
static CLOSURE_RET : [(Input , Action) ; 2] = [(Punct ("->") , SetState (& [(ExpectType , SetState (& BLOCK))])) , (Otherwise , SetState (& INIT)) ,] ;
};
}
