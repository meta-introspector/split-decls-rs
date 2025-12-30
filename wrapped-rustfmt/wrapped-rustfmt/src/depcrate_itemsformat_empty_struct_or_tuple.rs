// Generated macro for format_empty_struct_or_tuple (function)
macro_rules! Depcrate_itemsformat_empty_struct_or_tuple {
() => {
// Module: crate::items
// Provides: {"format_empty_struct_or_tuple"}
// Dependencies: {}
fn format_empty_struct_or_tuple (context : & RewriteContext < '_ > , span : Span , offset : Indent , result : & mut String , opener : & str , closer : & str ,) { let used_width = last_line_used_width (result , offset . width ()) + 3 ; if used_width > context . config . max_width () { result . push_str (& offset . to_string_with_newline (context . config)) } result . push_str (opener) ; let shape = Shape :: indented (offset . block_indent (context . config) , context . config) ; match rewrite_missing_comment (span , shape , context) { Ok (ref s) if s . is_empty () => () , Ok (ref s) => { let is_multi_line = ! is_single_line (s) ; if is_multi_line || first_line_contains_single_line_comment (s) { let nested_indent_str = offset . block_indent (context . config) . to_string_with_newline (context . config) ; result . push_str (& nested_indent_str) ; } result . push_str (s) ; if is_multi_line || last_line_contains_single_line_comment (s) { result . push_str (& offset . to_string_with_newline (context . config)) ; } } Err (_) => result . push_str (context . snippet (span)) , } result . push_str (closer) ; }
};
}
