// Generated macro for parse_lit_into_path (function)
macro_rules! Depcrate_internals_attrparse_lit_into_path {
() => {
// Module: crate::internals::attr
// Provides: {"parse_lit_into_path"}
// Dependencies: {}
fn parse_lit_into_path (cx : & Ctxt , attr_name : Symbol , meta : & ParseNestedMeta ,) -> syn :: Result < Option < syn :: Path > > { let Some (string) = get_lit_str (cx , attr_name , meta) ? else { return Ok (None) ; } ; Ok (match string . parse () { Ok (path) => Some (path) , Err (_) => { cx . error_spanned_by (& string , format ! ("failed to parse path: {:?}" , string . value ()) ,) ; None } }) }
};
}
