// Generated macro for IF_THEN (static)
macro_rules! Depcrate_scan_exprIF_THEN {
() => {
// Module: crate::scan_expr
// Provides: {"IF_THEN"}
// Dependencies: {}
static IF_THEN : [(Input , Action) ; 2] = [(Keyword ("else") , SetState (& IF_ELSE)) , (Otherwise , DecDepth)] ;
};
}
