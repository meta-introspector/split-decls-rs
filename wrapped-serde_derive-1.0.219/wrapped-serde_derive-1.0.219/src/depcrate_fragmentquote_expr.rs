// Generated macro for quote_expr (macro)
macro_rules! Depcrate_fragmentquote_expr {
() => {
// Module: crate::fragment
// Provides: {"quote_expr"}
// Dependencies: {}
macro_rules ! quote_expr { ($ ($ tt : tt) *) => { $ crate :: fragment :: Fragment :: Expr (quote ! ($ ($ tt) *)) } }
};
}
