// Generated macro for impl_130 (impl)
macro_rules! Depcrate_attrimpl_130 {
() => {
// Module: crate::attr
// Provides: {"impl_130"}
// Dependencies: {}
impl Rewrite for ast :: MetaItem { fn rewrite (& self , context : & RewriteContext < '_ > , shape : Shape) -> Option < String > { self . rewrite_result (context , shape) . ok () } fn rewrite_result (& self , context : & RewriteContext < '_ > , shape : Shape) -> RewriteResult { Ok (match self . kind { ast :: MetaItemKind :: Word => { rewrite_path (context , PathContext :: Type , & None , & self . path , shape) ? } ast :: MetaItemKind :: List (ref list) => { let path = rewrite_path (context , PathContext :: Type , & None , & self . path , shape) ? ; let has_trailing_comma = crate :: expr :: span_ends_with_comma (context , self . span) ; overflow :: rewrite_with_parens (context , & path , list . iter () , shape . sub_width (1 , self . span) ? , self . span , context . config . attr_fn_like_width () , Some (if has_trailing_comma { SeparatorTactic :: Always } else { SeparatorTactic :: Never }) ,) ? } ast :: MetaItemKind :: NameValue (ref lit) => { let path = rewrite_path (context , PathContext :: Type , & None , & self . path , shape) ? ; let lit_shape = shape . shrink_left (path . len () + 3 , self . span) ? ; let value = rewrite_literal (context , lit . as_token_lit () , lit . span , lit_shape) . unwrap_or_else (| _ | context . snippet (lit . span) . to_owned ()) ; format ! ("{path} = {value}") } }) } }
};
}
