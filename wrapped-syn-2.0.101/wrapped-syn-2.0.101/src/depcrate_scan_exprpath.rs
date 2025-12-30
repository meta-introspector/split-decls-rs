// Generated macro for PATH (static)
macro_rules! Depcrate_scan_exprPATH {
() => {
// Module: crate::scan_expr
// Provides: {"PATH"}
// Dependencies: {}
static PATH : [(Input , Action) ; 4] = [(Punct ("!=") , SetState (& INIT)) , (Punct ("!") , SetState (& INIT)) , (ConsumeNestedBrace , SetState (& IF_THEN)) , (Otherwise , SetState (& POSTFIX)) ,] ;
};
}
