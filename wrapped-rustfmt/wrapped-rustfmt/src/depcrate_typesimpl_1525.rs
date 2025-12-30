// Generated macro for impl_1525 (impl)
macro_rules! Depcrate_typesimpl_1525 {
() => {
// Module: crate::types
// Provides: {"impl_1525"}
// Dependencies: {}
impl Rewrite for ast :: GenericBounds { fn rewrite (& self , context : & RewriteContext < '_ > , shape : Shape) -> Option < String > { self . rewrite_result (context , shape) . ok () } fn rewrite_result (& self , context : & RewriteContext < '_ > , shape : Shape) -> RewriteResult { if self . is_empty () { return Ok (String :: new ()) ; } join_bounds (context , shape , self , true) } }
};
}
