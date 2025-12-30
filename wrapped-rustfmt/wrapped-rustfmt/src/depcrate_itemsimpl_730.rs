// Generated macro for impl_730 (impl)
macro_rules! Depcrate_itemsimpl_730 {
() => {
// Module: crate::items
// Provides: {"impl_730"}
// Dependencies: {}
impl Rewrite for ast :: FieldDef { fn rewrite (& self , context : & RewriteContext < '_ > , shape : Shape) -> Option < String > { self . rewrite_result (context , shape) . ok () } fn rewrite_result (& self , context : & RewriteContext < '_ > , shape : Shape) -> RewriteResult { rewrite_struct_field (context , self , shape , 0) } }
};
}
