// Generated macro for semicolon_for_expr (function)
macro_rules! Depcrate_utilssemicolon_for_expr {
() => {
// Module: crate::utils
// Provides: {"semicolon_for_expr"}
// Dependencies: {}
# [inline] pub (crate) fn semicolon_for_expr (context : & RewriteContext < '_ > , expr : & ast :: Expr) -> bool { if context . is_macro_def { return false ; } match expr . kind { ast :: ExprKind :: Ret (..) | ast :: ExprKind :: Continue (..) | ast :: ExprKind :: Break (..) => { context . config . trailing_semicolon () } _ => false , } }
};
}
