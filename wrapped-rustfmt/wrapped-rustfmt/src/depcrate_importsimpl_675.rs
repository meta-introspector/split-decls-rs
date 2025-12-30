// Generated macro for impl_675 (impl)
macro_rules! Depcrate_importsimpl_675 {
() => {
// Module: crate::imports
// Provides: {"impl_675"}
// Dependencies: {}
impl Rewrite for UseTree { fn rewrite (& self , context : & RewriteContext < '_ > , shape : Shape) -> Option < String > { self . rewrite_result (context , shape) . ok () } fn rewrite_result (& self , context : & RewriteContext < '_ > , mut shape : Shape) -> RewriteResult { let mut result = String :: with_capacity (256) ; let mut iter = self . path . iter () . peekable () ; while let Some (segment) = iter . next () { let segment_str = segment . rewrite_result (context , shape) ? ; result . push_str (& segment_str) ; if iter . peek () . is_some () { result . push_str ("::") ; shape = shape . offset_left (2 + segment_str . len () , self . span ()) ? ; } } Ok (result) } }
};
}
