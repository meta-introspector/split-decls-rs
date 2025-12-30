// Generated macro for impl_1543 (impl)
macro_rules! Depcrate_typesimpl_1543 {
() => {
// Module: crate::types
// Provides: {"impl_1543"}
// Dependencies: {}
impl Rewrite for ast :: GenericArg { fn rewrite (& self , context : & RewriteContext < '_ > , shape : Shape) -> Option < String > { self . rewrite_result (context , shape) . ok () } fn rewrite_result (& self , context : & RewriteContext < '_ > , shape : Shape) -> RewriteResult { match * self { ast :: GenericArg :: Lifetime (ref lt) => lt . rewrite_result (context , shape) , ast :: GenericArg :: Type (ref ty) => ty . rewrite_result (context , shape) , ast :: GenericArg :: Const (ref const_) => const_ . rewrite_result (context , shape) , } } }
};
}
