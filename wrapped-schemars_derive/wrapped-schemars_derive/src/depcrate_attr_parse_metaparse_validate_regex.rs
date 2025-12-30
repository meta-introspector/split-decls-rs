// Generated macro for parse_validate_regex (function)
macro_rules! Depcrate_attr_parse_metaparse_validate_regex {
() => {
// Module: crate::attr::parse_meta
// Provides: {"parse_validate_regex"}
// Dependencies: {}
pub fn parse_validate_regex (outer_meta : & CustomMeta , cx : & AttrCtxt) -> Result < Expr , () > { let mut path = None ; for nested_meta in parse_nested_meta (outer_meta , cx) ? { match path_str (nested_meta . path ()) . as_str () { "path" => match & path { Some (_) => cx . duplicate_error (& nested_meta) , None => path = parse_name_value_expr_handle_lit_str (nested_meta , cx) . ok () , } , "pattern" => { cx . error_spanned_by (nested_meta , "`pattern` is not supported in `validate(regex(...))` attribute - use either `validate(regex(path = ...))` or `schemars(regex(pattern = ...))` instead") ; } _ => { } } } path . ok_or_else (| | { cx . error_spanned_by (outer_meta , "`validate(regex(...))` attribute requires `path = ...`" ,) ; }) }
};
}
