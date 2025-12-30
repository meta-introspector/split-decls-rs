// Generated macro for parse_lit_into_ty (function)
macro_rules! Depcrate_internals_attrparse_lit_into_ty {
() => {
// Module: crate::internals::attr
// Provides: {"parse_lit_into_ty"}
// Dependencies: {}
fn parse_lit_into_ty (cx : & Ctxt , attr_name : Symbol , meta : & ParseNestedMeta ,) -> syn :: Result < Option < syn :: Type > > { let Some (string) = get_lit_str (cx , attr_name , meta) ? else { return Ok (None) ; } ; Ok (match string . parse () { Ok (ty) => Some (ty) , Err (_) => { cx . error_spanned_by (& string , format ! ("failed to parse type: {} = {:?}" , attr_name , string . value ()) ,) ; None } }) }
};
}
