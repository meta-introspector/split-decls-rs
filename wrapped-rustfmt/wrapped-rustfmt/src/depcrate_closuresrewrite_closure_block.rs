// Generated macro for rewrite_closure_block (function)
macro_rules! Depcrate_closuresrewrite_closure_block {
() => {
// Module: crate::closures
// Provides: {"rewrite_closure_block"}
// Dependencies: {}
fn rewrite_closure_block (block : & ast :: Expr , prefix : & str , context : & RewriteContext < '_ > , shape : Shape ,) -> RewriteResult { debug_assert ! (matches ! (block . kind , ast :: ExprKind :: Block (..)) , "expected a block expression") ; Ok (format ! ("{} {}" , prefix , block . rewrite_result (context , shape) ?)) }
};
}
