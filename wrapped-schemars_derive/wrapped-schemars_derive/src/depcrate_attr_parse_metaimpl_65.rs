// Generated macro for impl_65 (impl)
macro_rules! Depcrate_attr_parse_metaimpl_65 {
() => {
// Module: crate::attr::parse_meta
// Provides: {"impl_65"}
// Dependencies: {}
impl Parse for Extension { fn parse (input : ParseStream) -> syn :: Result < Self > { let key = input . parse :: < LitStr > () ? ; input . parse :: < Token ! [=] > () ? ; let mut value = TokenStream :: new () ; while ! input . is_empty () && ! input . peek (Token ! [,]) { value . extend ([input . parse :: < TokenTree > () ?]) ; } if value . is_empty () { return Err (syn :: Error :: new (input . span () , "Expected extension value")) ; } Ok (Extension { key_str : key . value () , key_lit : key , value , }) } }
};
}
