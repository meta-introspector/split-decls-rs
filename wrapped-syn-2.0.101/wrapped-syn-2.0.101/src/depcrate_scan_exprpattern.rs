// Generated macro for PATTERN (static)
macro_rules! Depcrate_scan_exprPATTERN {
() => {
// Module: crate::scan_expr
// Provides: {"PATTERN"}
// Dependencies: {}
static PATTERN : [(Input , Action) ; 15] = [(ConsumeDelimiter , SetState (& PATTERN)) , (Keyword ("box") , SetState (& PATTERN)) , (Keyword ("in") , IncDepth) , (Keyword ("mut") , SetState (& PATTERN)) , (Keyword ("ref") , SetState (& PATTERN)) , (Keyword ("_") , SetState (& PATTERN)) , (Punct ("!") , SetState (& PATTERN)) , (Punct ("&") , SetState (& PATTERN)) , (Punct ("..=") , SetState (& PATTERN)) , (Punct ("..") , SetState (& PATTERN)) , (Punct ("=") , SetState (& INIT)) , (Punct ("@") , SetState (& PATTERN)) , (Punct ("|") , SetState (& PATTERN)) , (ConsumeLiteral , SetState (& PATTERN)) , (ExpectPath , SetState (& PATTERN)) ,] ;
};
}
