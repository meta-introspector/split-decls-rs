// Generated macro for impl_495 (impl)
macro_rules! Depcrate_exprimpl_495 {
() => {
// Module: crate::expr
// Provides: {"impl_495"}
// Dependencies: {}
impl Rewrite for ast :: Expr { fn rewrite (& self , context : & RewriteContext < '_ > , shape : Shape) -> Option < String > { self . rewrite_result (context , shape) . ok () } fn rewrite_result (& self , context : & RewriteContext < '_ > , shape : Shape) -> RewriteResult { format_expr (self , ExprType :: SubExpression , context , shape) } }
};
}
