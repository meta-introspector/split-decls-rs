// Generated macro for stmt_is_expr (function)
macro_rules! Depcrate_exprstmt_is_expr {
() => {
// Module: crate::expr
// Provides: {"stmt_is_expr"}
// Dependencies: {}
pub (crate) fn stmt_is_expr (stmt : & ast :: Stmt) -> bool { matches ! (stmt . kind , ast :: StmtKind :: Expr (..)) }
};
}
