// Generated macro for parse_meta_list_with (function)
macro_rules! Depcrate_attr_parse_metaparse_meta_list_with {
() => {
// Module: crate::attr::parse_meta
// Provides: {"parse_meta_list_with"}
// Dependencies: {}
fn parse_meta_list_with < F : Parser > (meta : & CustomMeta , cx : & AttrCtxt , parser : F ,) -> Result < F :: Output , () > { let CustomMeta :: List (meta_list) = meta else { let name = path_str (meta . path ()) ; cx . error_spanned_by (meta , format_args ! ("expected {} {} attribute item to be of the form `{}(...)`" , cx . attr_type , name , name) ,) ; return Err (()) ; } ; meta_list . parse_args_with (parser) . map_err (| err | { cx . syn_error (err) ; }) }
};
}
