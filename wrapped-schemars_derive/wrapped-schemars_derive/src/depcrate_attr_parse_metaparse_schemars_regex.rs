// Generated macro for parse_schemars_regex (function)
macro_rules! Depcrate_attr_parse_metaparse_schemars_regex {
() => {
// Module: crate::attr::parse_meta
// Provides: {"parse_schemars_regex"}
// Dependencies: {}
pub fn parse_schemars_regex (outer_meta : & CustomMeta , cx : & AttrCtxt) -> Result < Expr , () > { let mut pattern = None ; for nested_meta in parse_nested_meta (outer_meta , cx) ? { match path_str (nested_meta . path ()) . as_str () { "pattern" => match & pattern { Some (_) => cx . duplicate_error (& nested_meta) , None => pattern = parse_name_value_expr (nested_meta , cx) . ok () , } , "path" => { cx . error_spanned_by (nested_meta , "`path` is not supported in `schemars(regex(...))` attribute - use `schemars(regex(pattern = ...))` instead") ; } unknown => { cx . error_spanned_by (nested_meta , format_args ! ("unknown item in schemars `regex` attribute: `{unknown}`") ,) ; } } } pattern . ok_or_else (| | { cx . error_spanned_by (outer_meta , "`schemars(regex(...))` attribute requires `pattern = ...`" ,) ; }) }
};
}
