// Generated macro for parse_lit_into_expr_path (function)
macro_rules! Depcrate_internals_attrparse_lit_into_expr_path {
() => {
// Module: crate::internals::attr
// Provides: {"parse_lit_into_expr_path"}
// Dependencies: {}
fn parse_lit_into_expr_path (cx : & Ctxt , attr_name : Symbol , meta : & ParseNestedMeta ,) -> syn :: Result < Option < syn :: ExprPath > > { let string = match get_lit_str (cx , attr_name , meta) ? { Some (string) => string , None => return Ok (None) , } ; Ok (match string . parse () { Ok (expr) => Some (expr) , Err (_) => { cx . error_spanned_by (& string , format ! ("failed to parse path: {:?}" , string . value ()) ,) ; None } }) }
};
}
