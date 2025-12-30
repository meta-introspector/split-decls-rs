// Generated macro for impl_1528 (impl)
macro_rules! Depcrate_typesimpl_1528 {
() => {
// Module: crate::types
// Provides: {"impl_1528"}
// Dependencies: {}
impl Rewrite for ast :: TraitRef { fn rewrite (& self , context : & RewriteContext < '_ > , shape : Shape) -> Option < String > { self . rewrite_result (context , shape) . ok () } fn rewrite_result (& self , context : & RewriteContext < '_ > , shape : Shape) -> RewriteResult { rewrite_path (context , PathContext :: Type , & None , & self . path , shape) } }
};
}
