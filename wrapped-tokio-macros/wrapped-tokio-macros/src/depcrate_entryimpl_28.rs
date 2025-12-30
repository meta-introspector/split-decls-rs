// Generated macro for impl_28 (impl)
macro_rules! Depcrate_entryimpl_28 {
() => {
// Module: crate::entry
// Provides: {"impl_28"}
// Dependencies: {}
impl Parse for ItemFn { # [inline] fn parse (input : ParseStream < '_ >) -> syn :: Result < Self > { let outer_attrs = input . call (Attribute :: parse_outer) ? ; let vis : Visibility = input . parse () ? ; let sig : Signature = input . parse () ? ; let content ; let brace_token = braced ! (content in input) ; let inner_attrs = Attribute :: parse_inner (& content) ? ; let mut buf = proc_macro2 :: TokenStream :: new () ; let mut stmts = Vec :: new () ; while ! content . is_empty () { if let Some (semi) = content . parse :: < Option < syn :: Token ! [;] > > () ? { semi . to_tokens (& mut buf) ; stmts . push (buf) ; buf = proc_macro2 :: TokenStream :: new () ; continue ; } buf . extend ([content . parse :: < TokenTree > () ?]) ; } if ! buf . is_empty () { stmts . push (buf) ; } Ok (Self { outer_attrs , vis , sig , brace_token , inner_attrs , stmts , }) } }
};
}
