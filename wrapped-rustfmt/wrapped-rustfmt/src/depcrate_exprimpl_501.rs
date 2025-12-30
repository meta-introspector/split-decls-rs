// Generated macro for impl_501 (impl)
macro_rules! Depcrate_exprimpl_501 {
() => {
// Module: crate::expr
// Provides: {"impl_501"}
// Dependencies: {}
impl Rewrite for ast :: Block { fn rewrite (& self , context : & RewriteContext < '_ > , shape : Shape) -> Option < String > { self . rewrite_result (context , shape) . ok () } fn rewrite_result (& self , context : & RewriteContext < '_ > , shape : Shape) -> RewriteResult { rewrite_block (self , None , None , context , shape) } }
};
}
