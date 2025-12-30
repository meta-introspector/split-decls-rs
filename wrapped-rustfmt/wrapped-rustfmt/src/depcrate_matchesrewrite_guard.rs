// Generated macro for rewrite_guard (function)
macro_rules! Depcrate_matchesrewrite_guard {
() => {
// Module: crate::matches
// Provides: {"rewrite_guard"}
// Dependencies: {}
fn rewrite_guard (context : & RewriteContext < '_ > , guard : & Option < ptr :: P < ast :: Expr > > , shape : Shape , pattern_width : usize , multiline_pattern : bool ,) -> RewriteResult { if let Some (ref guard) = * guard { let cond_shape = shape . offset_left_opt (pattern_width + 4) . and_then (| s | s . sub_width_opt (5)) ; if ! multiline_pattern { if let Some (cond_shape) = cond_shape { if let Ok (cond_str) = guard . rewrite_result (context , cond_shape) { if ! cond_str . contains ('\n') || pattern_width <= context . config . tab_spaces () { return Ok (format ! (" if {cond_str}")) ; } } } } let cond_shape = Shape :: indented (shape . indent . block_indent (context . config) , context . config) . offset_left (3 , guard . span) ? . sub_width (5 , guard . span) ? ; let cond_str = guard . rewrite_result (context , cond_shape) ? ; Ok (format ! ("{}if {}" , cond_shape . indent . to_string_with_newline (context . config) , cond_str)) } else { Ok (String :: new ()) } }
};
}
