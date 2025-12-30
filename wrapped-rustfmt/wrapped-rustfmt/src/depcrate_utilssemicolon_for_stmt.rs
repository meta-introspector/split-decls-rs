// Generated macro for semicolon_for_stmt (function)
macro_rules! Depcrate_utilssemicolon_for_stmt {
() => {
// Module: crate::utils
// Provides: {"semicolon_for_stmt"}
// Dependencies: {}
# [inline] pub (crate) fn semicolon_for_stmt (context : & RewriteContext < '_ > , stmt : & ast :: Stmt , is_last_expr : bool ,) -> bool { match stmt . kind { ast :: StmtKind :: Semi (ref expr) => match expr . kind { ast :: ExprKind :: While (..) | ast :: ExprKind :: Loop (..) | ast :: ExprKind :: ForLoop { .. } => { false } ast :: ExprKind :: Break (..) | ast :: ExprKind :: Continue (..) | ast :: ExprKind :: Ret (..) => { context . config . trailing_semicolon () || ! is_last_expr } _ => true , } , ast :: StmtKind :: Expr (..) => false , _ => true , } }
};
}
