// Generated macro for REFERENCE (static)
macro_rules! Depcrate_scan_exprREFERENCE {
() => {
// Module: crate::scan_expr
// Provides: {"REFERENCE"}
// Dependencies: {}
static REFERENCE : [(Input , Action) ; 3] = [(Keyword ("mut") , SetState (& INIT)) , (Keyword ("raw") , SetState (& RAW)) , (Otherwise , SetState (& INIT)) ,] ;
};
}
