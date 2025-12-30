// Generated macro for stmt_expr (function)
macro_rules! Depcrate_utilsstmt_expr {
() => {
// Module: crate::utils
// Provides: {"stmt_expr"}
// Dependencies: {}
# [inline] pub (crate) fn stmt_expr (stmt : & ast :: Stmt) -> Option < & ast :: Expr > { match stmt . kind { ast :: StmtKind :: Expr (ref expr) => Some (expr) , _ => None , } }
};
}
