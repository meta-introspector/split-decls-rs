// Generated macro for impl_128 (impl)
macro_rules! Depcrate_attrimpl_128 {
() => {
// Module: crate::attr
// Provides: {"impl_128"}
// Dependencies: {}
impl Rewrite for ast :: MetaItemInner { fn rewrite (& self , context : & RewriteContext < '_ > , shape : Shape) -> Option < String > { self . rewrite_result (context , shape) . ok () } fn rewrite_result (& self , context : & RewriteContext < '_ > , shape : Shape) -> RewriteResult { match self { ast :: MetaItemInner :: MetaItem (ref meta_item) => meta_item . rewrite_result (context , shape) , ast :: MetaItemInner :: Lit (ref l) => { rewrite_literal (context , l . as_token_lit () , l . span , shape) } } } }
};
}
