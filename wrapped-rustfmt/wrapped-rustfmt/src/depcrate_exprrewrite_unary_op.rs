// Generated macro for rewrite_unary_op (function)
macro_rules! Depcrate_exprrewrite_unary_op {
() => {
// Module: crate::expr
// Provides: {"rewrite_unary_op"}
// Dependencies: {}
fn rewrite_unary_op (context : & RewriteContext < '_ > , op : ast :: UnOp , expr : & ast :: Expr , shape : Shape ,) -> RewriteResult { rewrite_unary_prefix (context , op . as_str () , expr , shape) }
};
}
