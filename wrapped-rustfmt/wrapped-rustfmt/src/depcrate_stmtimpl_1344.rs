// Generated macro for impl_1344 (impl)
macro_rules! Depcrate_stmtimpl_1344 {
() => {
// Module: crate::stmt
// Provides: {"impl_1344"}
// Dependencies: {}
impl < 'a > Rewrite for Stmt < 'a > { fn rewrite (& self , context : & RewriteContext < '_ > , shape : Shape) -> Option < String > { self . rewrite_result (context , shape) . ok () } fn rewrite_result (& self , context : & RewriteContext < '_ > , shape : Shape ,) -> crate :: rewrite :: RewriteResult { let expr_type = if context . config . style_edition () >= StyleEdition :: Edition2024 && self . is_last_expr () { ExprType :: SubExpression } else { ExprType :: Statement } ; format_stmt (context , shape , self . as_ast_node () , expr_type , self . is_last_expr () ,) } }
};
}
