// Generated macro for wrap_struct_field (function)
macro_rules! Depcrate_exprwrap_struct_field {
() => {
// Module: crate::expr
// Provides: {"wrap_struct_field"}
// Dependencies: {}
pub (crate) fn wrap_struct_field (context : & RewriteContext < '_ > , attrs : & [ast :: Attribute] , fields_str : & str , shape : Shape , nested_shape : Shape , one_line_width : usize ,) -> RewriteResult { let should_vertical = context . config . indent_style () == IndentStyle :: Block && (fields_str . contains ('\n') || ! context . config . struct_lit_single_line () || fields_str . len () > one_line_width) ; let inner_attrs = & inner_attributes (attrs) ; if inner_attrs . is_empty () { if should_vertical { Ok (format ! ("{}{}{}" , nested_shape . indent . to_string_with_newline (context . config) , fields_str , shape . indent . to_string_with_newline (context . config))) } else { Ok (format ! (" {fields_str} ")) } } else { Ok (format ! ("{}{}{}{}{}" , nested_shape . indent . to_string_with_newline (context . config) , inner_attrs . rewrite_result (context , shape) ?, nested_shape . indent . to_string_with_newline (context . config) , fields_str , shape . indent . to_string_with_newline (context . config))) } }
};
}
