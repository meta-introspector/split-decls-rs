// Generated macro for impl_836 (impl)
macro_rules! Depcrate_macrosimpl_836 {
() => {
// Module: crate::macros
// Provides: {"impl_836"}
// Dependencies: {}
impl Rewrite for ast :: Item { fn rewrite (& self , context : & RewriteContext < '_ > , shape : Shape) -> Option < String > { self . rewrite_result (context , shape) . ok () } fn rewrite_result (& self , context : & RewriteContext < '_ > , shape : Shape) -> RewriteResult { let mut visitor = crate :: visitor :: FmtVisitor :: from_context (context) ; visitor . block_indent = shape . indent ; visitor . last_pos = self . span () . lo () ; visitor . visit_item (self) ; Ok (visitor . buffer . to_owned ()) } }
};
}
