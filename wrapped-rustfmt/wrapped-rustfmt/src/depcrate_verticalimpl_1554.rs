// Generated macro for impl_1554 (impl)
macro_rules! Depcrate_verticalimpl_1554 {
() => {
// Module: crate::vertical
// Provides: {"impl_1554"}
// Dependencies: {}
impl AlignedItem for ast :: FieldDef { fn skip (& self) -> bool { contains_skip (& self . attrs) } fn get_span (& self) -> Span { self . span () } fn rewrite_prefix (& self , context : & RewriteContext < '_ > , shape : Shape) -> RewriteResult { let attrs_str = self . attrs . rewrite_result (context , shape) ? ; let missing_span = if self . attrs . is_empty () { mk_sp (self . span . lo () , self . span . lo ()) } else { mk_sp (self . attrs . last () . unwrap () . span . hi () , self . span . lo ()) } ; let attrs_extendable = self . ident . is_none () && is_attributes_extendable (& attrs_str) ; let field_str = rewrite_struct_field_prefix (context , self) ? ; combine_strs_with_missing_comments (context , & attrs_str , & field_str , missing_span , shape , attrs_extendable ,) } fn rewrite_aligned_item (& self , context : & RewriteContext < '_ > , shape : Shape , prefix_max_width : usize ,) -> RewriteResult { rewrite_struct_field (context , self , shape , prefix_max_width) } }
};
}
