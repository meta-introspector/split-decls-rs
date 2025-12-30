// Generated macro for impl_1555 (impl)
macro_rules! Depcrate_verticalimpl_1555 {
() => {
// Module: crate::vertical
// Provides: {"impl_1555"}
// Dependencies: {}
impl AlignedItem for ast :: ExprField { fn skip (& self) -> bool { contains_skip (& self . attrs) } fn get_span (& self) -> Span { self . span () } fn rewrite_prefix (& self , context : & RewriteContext < '_ > , shape : Shape) -> RewriteResult { let attrs_str = self . attrs . rewrite_result (context , shape) ? ; let name = rewrite_ident (context , self . ident) ; let missing_span = if self . attrs . is_empty () { mk_sp (self . span . lo () , self . span . lo ()) } else { mk_sp (self . attrs . last () . unwrap () . span . hi () , self . span . lo ()) } ; combine_strs_with_missing_comments (context , & attrs_str , name , missing_span , shape , is_attributes_extendable (& attrs_str) ,) } fn rewrite_aligned_item (& self , context : & RewriteContext < '_ > , shape : Shape , prefix_max_width : usize ,) -> RewriteResult { rewrite_field (context , self , shape , prefix_max_width) } }
};
}
