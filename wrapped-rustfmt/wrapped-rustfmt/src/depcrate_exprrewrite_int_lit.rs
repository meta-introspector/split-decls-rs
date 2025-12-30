// Generated macro for rewrite_int_lit (function)
macro_rules! Depcrate_exprrewrite_int_lit {
() => {
// Module: crate::expr
// Provides: {"rewrite_int_lit"}
// Dependencies: {}
fn rewrite_int_lit (context : & RewriteContext < '_ > , token_lit : token :: Lit , span : Span , shape : Shape ,) -> RewriteResult { if token_lit . is_semantic_float () { return rewrite_float_lit (context , token_lit , span , shape) ; } let symbol = token_lit . symbol . as_str () ; if let Some (symbol_stripped) = symbol . strip_prefix ("0x") { let hex_lit = match context . config . hex_literal_case () { HexLiteralCase :: Preserve => None , HexLiteralCase :: Upper => Some (symbol_stripped . to_ascii_uppercase ()) , HexLiteralCase :: Lower => Some (symbol_stripped . to_ascii_lowercase ()) , } ; if let Some (hex_lit) = hex_lit { return wrap_str (format ! ("0x{}{}" , hex_lit , token_lit . suffix . as_ref () . map_or ("" , | s | s . as_str ())) , context . config . max_width () , shape ,) . max_width_error (shape . width , span) ; } } wrap_str (context . snippet (span) . to_owned () , context . config . max_width () , shape ,) . max_width_error (shape . width , span) }
};
}
