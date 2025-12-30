// Generated macro for impl_743 (impl)
macro_rules! Depcrate_itemsimpl_743 {
() => {
// Module: crate::items
// Provides: {"impl_743"}
// Dependencies: {}
impl < 'a > Rewrite for OpaqueType < 'a > { fn rewrite (& self , context : & RewriteContext < '_ > , shape : Shape) -> Option < String > { let shape = shape . offset_left_opt (5) ? ; self . bounds . rewrite (context , shape) . map (| s | format ! ("impl {}" , s)) } }
};
}
