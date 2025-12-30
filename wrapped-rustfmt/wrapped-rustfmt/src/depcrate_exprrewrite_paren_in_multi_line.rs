// Generated macro for rewrite_paren_in_multi_line (function)
macro_rules! Depcrate_exprrewrite_paren_in_multi_line {
() => {
// Module: crate::expr
// Provides: {"rewrite_paren_in_multi_line"}
// Dependencies: {}
fn rewrite_paren_in_multi_line (context : & RewriteContext < '_ > , subexpr : & ast :: Expr , shape : Shape , pre_span : Span , post_span : Span ,) -> RewriteResult { let nested_indent = shape . indent . block_indent (context . config) ; let nested_shape = Shape :: indented (nested_indent , context . config) ; let pre_comment = rewrite_missing_comment (pre_span , nested_shape , context) ? ; let post_comment = rewrite_missing_comment (post_span , nested_shape , context) ? ; let subexpr_str = subexpr . rewrite_result (context , nested_shape) ? ; let mut result = String :: with_capacity (subexpr_str . len () * 2) ; result . push ('(') ; if ! pre_comment . is_empty () { result . push_str (& nested_indent . to_string_with_newline (context . config)) ; result . push_str (& pre_comment) ; } result . push_str (& nested_indent . to_string_with_newline (context . config)) ; result . push_str (& subexpr_str) ; if ! post_comment . is_empty () { result . push_str (& nested_indent . to_string_with_newline (context . config)) ; result . push_str (& post_comment) ; } result . push_str (& shape . indent . to_string_with_newline (context . config)) ; result . push (')') ; Ok (result) }
};
}
