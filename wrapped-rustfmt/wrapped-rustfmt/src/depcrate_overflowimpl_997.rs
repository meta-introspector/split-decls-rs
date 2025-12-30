// Generated macro for impl_997 (impl)
macro_rules! Depcrate_overflowimpl_997 {
() => {
// Module: crate::overflow
// Provides: {"impl_997"}
// Dependencies: {}
impl < 'a > Rewrite for OverflowableItem < 'a > { fn rewrite (& self , context : & RewriteContext < '_ > , shape : Shape) -> Option < String > { self . map (| item | item . rewrite (context , shape)) } fn rewrite_result (& self , context : & RewriteContext < '_ > , shape : Shape) -> RewriteResult { self . map (| item | item . rewrite_result (context , shape)) } }
};
}
