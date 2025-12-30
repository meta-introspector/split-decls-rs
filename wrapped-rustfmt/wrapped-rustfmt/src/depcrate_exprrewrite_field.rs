// Generated macro for rewrite_field (function)
macro_rules! Depcrate_exprrewrite_field {
() => {
// Module: crate::expr
// Provides: {"rewrite_field"}
// Dependencies: {}
pub (crate) fn rewrite_field (context : & RewriteContext < '_ > , field : & ast :: ExprField , shape : Shape , prefix_max_width : usize ,) -> RewriteResult { if contains_skip (& field . attrs) { return Ok (context . snippet (field . span ()) . to_owned ()) ; } let mut attrs_str = field . attrs . rewrite_result (context , shape) ? ; if ! attrs_str . is_empty () { attrs_str . push_str (& shape . indent . to_string_with_newline (context . config)) ; } ; let name = context . snippet (field . ident . span) ; if field . is_shorthand { Ok (attrs_str + name) } else { let mut separator = String :: from (struct_lit_field_separator (context . config)) ; for _ in 0 .. prefix_max_width . saturating_sub (name . len ()) { separator . push (' ') ; } let overhead = name . len () + separator . len () ; let expr_shape = shape . offset_left (overhead , field . span) ? ; let expr = field . expr . rewrite_result (context , expr_shape) ; let is_lit = matches ! (field . expr . kind , ast :: ExprKind :: Lit (_)) ; match expr { Ok (ref e) if ! is_lit && e . as_str () == name && context . config . use_field_init_shorthand () => { Ok (attrs_str + name) } Ok (e) => Ok (format ! ("{attrs_str}{name}{separator}{e}")) , Err (_) => { let expr_offset = shape . indent . block_indent (context . config) ; let expr = field . expr . rewrite_result (context , Shape :: indented (expr_offset , context . config)) ; expr . map (| s | { format ! ("{}{}:\n{}{}" , attrs_str , name , expr_offset . to_string (context . config) , s) }) } } } }
};
}
