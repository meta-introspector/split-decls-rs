// Generated macro for POSTFIX (static)
macro_rules! Depcrate_scan_exprPOSTFIX {
() => {
// Module: crate::scan_expr
// Provides: {"POSTFIX"}
// Dependencies: {}
static POSTFIX : [(Input , Action) ; 10] = [(Keyword ("as") , SetState (& [(ExpectType , SetState (& POSTFIX))])) , (Punct ("..=") , SetState (& INIT)) , (Punct ("..") , SetState (& RANGE)) , (Punct (".") , SetState (& DOT)) , (Punct ("?") , SetState (& POSTFIX)) , (ConsumeBinOp , SetState (& INIT)) , (Punct ("=") , SetState (& INIT)) , (ConsumeNestedBrace , SetState (& IF_THEN)) , (ConsumeDelimiter , SetState (& POSTFIX)) , (Empty , Finish) ,] ;
};
}
