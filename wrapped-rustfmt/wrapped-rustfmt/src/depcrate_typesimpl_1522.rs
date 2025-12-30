// Generated macro for impl_1522 (impl)
macro_rules! Depcrate_typesimpl_1522 {
() => {
// Module: crate::types
// Provides: {"impl_1522"}
// Dependencies: {}
impl Rewrite for ast :: AnonConst { fn rewrite (& self , context : & RewriteContext < '_ > , shape : Shape) -> Option < String > { self . rewrite_result (context , shape) . ok () } fn rewrite_result (& self , context : & RewriteContext < '_ > , shape : Shape) -> RewriteResult { format_expr (& self . value , ExprType :: SubExpression , context , shape) } }
};
}
