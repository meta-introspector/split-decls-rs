// Generated macro for impl_1535 (impl)
macro_rules! Depcrate_typesimpl_1535 {
() => {
// Module: crate::types
// Provides: {"impl_1535"}
// Dependencies: {}
impl Rewrite for ast :: PreciseCapturingArg { fn rewrite (& self , context : & RewriteContext < '_ > , shape : Shape) -> Option < String > { self . rewrite_result (context , shape) . ok () } fn rewrite_result (& self , context : & RewriteContext < '_ > , shape : Shape) -> RewriteResult { match self { ast :: PreciseCapturingArg :: Lifetime (lt) => lt . rewrite_result (context , shape) , ast :: PreciseCapturingArg :: Arg (p , _) => { rewrite_path (context , PathContext :: Type , & None , p , shape) } } } }
};
}
