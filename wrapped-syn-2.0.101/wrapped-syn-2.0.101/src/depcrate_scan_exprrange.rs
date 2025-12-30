// Generated macro for RANGE (static)
macro_rules! Depcrate_scan_exprRANGE {
() => {
// Module: crate::scan_expr
// Provides: {"RANGE"}
// Dependencies: {}
static RANGE : [(Input , Action) ; 6] = [(Punct ("..=") , SetState (& INIT)) , (Punct ("..") , SetState (& RANGE)) , (Punct (".") , SetState (& DOT)) , (ConsumeNestedBrace , SetState (& IF_THEN)) , (Empty , Finish) , (Otherwise , SetState (& INIT)) ,] ;
};
}
