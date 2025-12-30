// Generated macro for impl_736 (impl)
macro_rules! Depcrate_itemsimpl_736 {
() => {
// Module: crate::items
// Provides: {"impl_736"}
// Dependencies: {}
impl < 'a > Rewrite for OpaqueType < 'a > { fn rewrite (& self , context : & RewriteContext < '_ > , shape : Shape) -> Option < String > { let shape = shape . offset_left (5) ? ; self . bounds . rewrite (context , shape) . map (| s | format ! ("impl {}" , s)) } }
};
}
