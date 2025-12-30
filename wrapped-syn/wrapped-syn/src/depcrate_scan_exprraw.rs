// Generated macro for RAW (static)
macro_rules! Depcrate_scan_exprRAW {
() => {
// Module: crate::scan_expr
// Provides: {"RAW"}
// Dependencies: {}
static RAW : [(Input , Action) ; 3] = [(Keyword ("const") , SetState (& INIT)) , (Keyword ("mut") , SetState (& INIT)) , (Otherwise , SetState (& POSTFIX)) ,] ;
};
}
