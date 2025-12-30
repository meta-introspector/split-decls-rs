// Generated macro for try_rewrite_without_block (function)
macro_rules! Depcrate_closurestry_rewrite_without_block {
() => {
// Module: crate::closures
// Provides: {"try_rewrite_without_block"}
// Dependencies: {}
fn try_rewrite_without_block (expr : & ast :: Expr , prefix : & str , context : & RewriteContext < '_ > , shape : Shape , body_shape : Shape ,) -> RewriteResult { let expr = get_inner_expr (expr , prefix , context) ; if is_block_closure_forced (context , expr) { rewrite_closure_with_block (expr , prefix , context , shape) } else { rewrite_closure_expr (expr , prefix , context , body_shape) } }
};
}
