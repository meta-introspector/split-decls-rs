// Generated macro for parse_lit_into_lifetimes (function)
macro_rules! Depcrate_internals_attrparse_lit_into_lifetimes {
() => {
// Module: crate::internals::attr
// Provides: {"parse_lit_into_lifetimes"}
// Dependencies: {}
fn parse_lit_into_lifetimes (cx : & Ctxt , meta : & ParseNestedMeta ,) -> syn :: Result < BTreeSet < syn :: Lifetime > > { let Some (string) = get_lit_str (cx , BORROW , meta) ? else { return Ok (BTreeSet :: new ()) ; } ; if let Ok (lifetimes) = string . parse_with (| input : ParseStream | { let mut set = BTreeSet :: new () ; while ! input . is_empty () { let lifetime : Lifetime = input . parse () ? ; if ! set . insert (lifetime . clone ()) { cx . error_spanned_by (& string , format ! ("duplicate borrowed lifetime `{}`" , lifetime) ,) ; } if input . is_empty () { break ; } input . parse :: < Token ! [+] > () ? ; } Ok (set) }) { if lifetimes . is_empty () { cx . error_spanned_by (string , "at least one lifetime must be borrowed") ; } return Ok (lifetimes) ; } cx . error_spanned_by (& string , format ! ("failed to parse borrowed lifetimes: {:?}" , string . value ()) ,) ; Ok (BTreeSet :: new ()) }
};
}
