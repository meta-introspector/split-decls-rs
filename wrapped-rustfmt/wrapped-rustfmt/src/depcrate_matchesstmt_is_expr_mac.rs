// Generated macro for stmt_is_expr_mac (function)
macro_rules! Depcrate_matchesstmt_is_expr_mac {
() => {
// Module: crate::matches
// Provides: {"stmt_is_expr_mac"}
// Dependencies: {}
fn stmt_is_expr_mac (stmt : & ast :: Stmt) -> bool { if let ast :: StmtKind :: Expr (expr) = & stmt . kind { if let ast :: ExprKind :: MacCall (_) = & expr . kind { return true ; } } false }
};
}
