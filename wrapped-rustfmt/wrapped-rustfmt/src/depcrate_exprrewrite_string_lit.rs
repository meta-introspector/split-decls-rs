// Generated macro for rewrite_string_lit (function)
macro_rules! Depcrate_exprrewrite_string_lit {
() => {
// Module: crate::expr
// Provides: {"rewrite_string_lit"}
// Dependencies: {}
fn rewrite_string_lit (context : & RewriteContext < '_ > , span : Span , shape : Shape) -> RewriteResult { let string_lit = context . snippet (span) ; if ! context . config . format_strings () { if string_lit . lines () . dropping_back (1) . all (| line | line . ends_with ('\\')) && context . config . style_edition () >= StyleEdition :: Edition2024 { return Ok (string_lit . to_owned ()) ; } else { return wrap_str (string_lit . to_owned () , context . config . max_width () , shape) . max_width_error (shape . width , span) ; } } let str_lit = & string_lit [1 .. string_lit . len () - 1] ; rewrite_string (str_lit , & StringFormat :: new (shape . visual_indent (0) , context . config) , shape . width . saturating_sub (2) ,) . max_width_error (shape . width , span) }
};
}
