// Generated macro for parse_length_or_range (function)
macro_rules! Depcrate_attr_parse_metaparse_length_or_range {
() => {
// Module: crate::attr::parse_meta
// Provides: {"parse_length_or_range"}
// Dependencies: {}
pub fn parse_length_or_range (outer_meta : & CustomMeta , cx : & AttrCtxt) -> Result < LengthOrRange , () > { let outer_name = path_str (outer_meta . path ()) ; let mut result = LengthOrRange :: default () ; for nested_meta in parse_nested_meta (outer_meta , cx) ? { match path_str (nested_meta . path ()) . as_str () { "min" => match (& result . min , & result . equal) { (Some (_) , _) => cx . duplicate_error (& nested_meta) , (_ , Some (_)) => cx . mutual_exclusive_error (& nested_meta , "equal") , _ => result . min = parse_name_value_expr_handle_lit_str (nested_meta , cx) . ok () , } , "max" => match (& result . max , & result . equal) { (Some (_) , _) => cx . duplicate_error (& nested_meta) , (_ , Some (_)) => cx . mutual_exclusive_error (& nested_meta , "equal") , _ => result . max = parse_name_value_expr_handle_lit_str (nested_meta , cx) . ok () , } , "equal" => match (& result . min , & result . max , & result . equal) { (Some (_) , _ , _) => cx . mutual_exclusive_error (& nested_meta , "min") , (_ , Some (_) , _) => cx . mutual_exclusive_error (& nested_meta , "max") , (_ , _ , Some (_)) => cx . duplicate_error (& nested_meta) , _ => result . equal = parse_name_value_expr_handle_lit_str (nested_meta , cx) . ok () , } , unknown => { if cx . attr_type == "schemars" { cx . error_spanned_by (nested_meta , format_args ! ("unknown item in schemars {outer_name} attribute: `{unknown}`" ,) ,) ; } } } } Ok (result) }
};
}
