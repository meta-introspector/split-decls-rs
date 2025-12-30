// Generated macro for get_inner_expr (function)
macro_rules! Depcrate_closuresget_inner_expr {
() => {
// Module: crate::closures
// Provides: {"get_inner_expr"}
// Dependencies: {}
fn get_inner_expr < 'a > (expr : & 'a ast :: Expr , prefix : & str , context : & RewriteContext < '_ > ,) -> & 'a ast :: Expr { if let ast :: ExprKind :: Block (ref block , ref label) = expr . kind { if ! needs_block (block , label , prefix , context) { if let Some (expr) = block . stmts . first () . and_then (stmt_expr) { return get_inner_expr (expr , prefix , context) ; } } } expr }
};
}
