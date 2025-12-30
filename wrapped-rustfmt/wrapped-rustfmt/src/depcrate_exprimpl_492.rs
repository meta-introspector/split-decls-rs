// Generated macro for impl_492 (impl)
macro_rules! Depcrate_exprimpl_492 {
() => {
// Module: crate::expr
// Provides: {"impl_492"}
// Dependencies: {}
impl Rewrite for ast :: Expr { fn rewrite (& self , context : & RewriteContext < '_ > , shape : Shape) -> Option < String > { self . rewrite_result (context , shape) . ok () } fn rewrite_result (& self , context : & RewriteContext < '_ > , shape : Shape) -> RewriteResult { format_expr (self , ExprType :: SubExpression , context , shape) } }
};
}
