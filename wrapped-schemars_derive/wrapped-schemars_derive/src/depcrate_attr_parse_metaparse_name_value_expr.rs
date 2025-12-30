// Generated macro for parse_name_value_expr (function)
macro_rules! Depcrate_attr_parse_metaparse_name_value_expr {
() => {
// Module: crate::attr::parse_meta
// Provides: {"parse_name_value_expr"}
// Dependencies: {}
pub fn parse_name_value_expr (meta : CustomMeta , cx : & AttrCtxt) -> Result < Expr , () > { if let CustomMeta :: NameValue (m) = meta { Ok (m . value) } else { let name = path_str (meta . path ()) ; cx . error_spanned_by (meta , format_args ! ("expected {} {} attribute item to have a value: `{} = ...`" , cx . attr_type , name , name) ,) ; Err (()) } }
};
}
