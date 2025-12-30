// Generated macro for choose_rhs (function)
macro_rules! Depcrate_exprchoose_rhs {
() => {
// Module: crate::expr
// Provides: {"choose_rhs"}
// Dependencies: {}
fn choose_rhs < R : Rewrite > (context : & RewriteContext < '_ > , expr : & R , shape : Shape , orig_rhs : RewriteResult , _rhs_kind : & RhsAssignKind < '_ > , rhs_tactics : RhsTactics , has_rhs_comment : bool ,) -> RewriteResult { match orig_rhs { Ok (ref new_str) if new_str . is_empty () => Ok (String :: new ()) , Ok (ref new_str) if ! new_str . contains ('\n') && unicode_str_width (new_str) <= shape . width => { Ok (format ! (" {new_str}")) } _ => { let new_shape = shape_from_rhs_tactic (context , shape , rhs_tactics) . unknown_error () ? ; let new_rhs = expr . rewrite_result (context , new_shape) ; let new_indent_str = & shape . indent . block_indent (context . config) . to_string_with_newline (context . config) ; let before_space_str = if has_rhs_comment { "" } else { " " } ; match (orig_rhs , new_rhs) { (Ok (ref orig_rhs) , Ok (ref new_rhs)) if ! filtered_str_fits (& new_rhs , context . config . max_width () , new_shape) => { Ok (format ! ("{before_space_str}{orig_rhs}")) } (Ok (ref orig_rhs) , Ok (ref new_rhs)) if prefer_next_line (orig_rhs , new_rhs , rhs_tactics) => { Ok (format ! ("{new_indent_str}{new_rhs}")) } (Err (_) , Ok (ref new_rhs)) => Ok (format ! ("{new_indent_str}{new_rhs}")) , (Err (_) , Err (_)) if rhs_tactics == RhsTactics :: AllowOverflow => { let shape = shape . infinite_width () ; expr . rewrite_result (context , shape) . map (| s | format ! ("{}{}" , before_space_str , s)) } (Err (_) , Err (new_rhs_err)) => Err (new_rhs_err) , (Ok (orig_rhs) , _) => Ok (format ! ("{before_space_str}{orig_rhs}")) , } } } }
};
}
