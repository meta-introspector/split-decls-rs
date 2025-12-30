// Generated macro for require_name_value_lit_str (function)
macro_rules! Depcrate_attr_parse_metarequire_name_value_lit_str {
() => {
// Module: crate::attr::parse_meta
// Provides: {"require_name_value_lit_str"}
// Dependencies: {}
pub fn require_name_value_lit_str (meta : CustomMeta , cx : & AttrCtxt) -> Result < LitStr , () > { if let CustomMeta :: NameValue (MetaNameValue { value : Expr :: Lit (ExprLit { lit : Lit :: Str (lit_str) , .. }) , .. }) = meta { Ok (lit_str) } else { let name = path_str (meta . path ()) ; cx . error_spanned_by (meta , format_args ! ("expected {} {} attribute item to have a string value: `{} = \"...\"`" , cx . attr_type , name , name) ,) ; Err (()) } }
};
}
