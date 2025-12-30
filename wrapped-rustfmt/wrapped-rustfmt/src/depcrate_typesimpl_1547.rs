// Generated macro for impl_1547 (impl)
macro_rules! Depcrate_typesimpl_1547 {
() => {
// Module: crate::types
// Provides: {"impl_1547"}
// Dependencies: {}
impl Rewrite for ast :: Lifetime { fn rewrite (& self , context : & RewriteContext < '_ > , shape : Shape) -> Option < String > { self . rewrite_result (context , shape) . ok () } fn rewrite_result (& self , context : & RewriteContext < '_ > , _ : Shape) -> RewriteResult { Ok (context . snippet (self . ident . span) . to_owned ()) } }
};
}
