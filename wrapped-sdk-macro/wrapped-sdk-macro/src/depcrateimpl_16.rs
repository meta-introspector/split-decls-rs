// Generated macro for impl_16 (impl)
macro_rules! Depcrateimpl_16 {
() => {
// Module: crate
// Provides: {"impl_16"}
// Dependencies: {}
impl Parse for Pubkeys { fn parse (input : ParseStream) -> Result < Self > { let pubkey_type = quote ! { :: solana_sdk :: pubkey :: Pubkey } ; let method = input . parse () ? ; let _comma : Token ! [,] = input . parse () ? ; let (num , pubkeys) = if input . peek (syn :: LitStr) { let id_literal : LitStr = input . parse () ? ; (1 , parse_pubkey (& id_literal , & pubkey_type) ?) } else if input . peek (Bracket) { let pubkey_strings ; bracketed ! (pubkey_strings in input) ; let punctuated : Punctuated < LitStr , Token ! [,] > = Punctuated :: parse_terminated (& pubkey_strings) ? ; let mut pubkeys : Punctuated < proc_macro2 :: TokenStream , Token ! [,] > = Punctuated :: new () ; for string in punctuated . iter () { pubkeys . push (parse_pubkey (string , & pubkey_type) ?) ; } (pubkeys . len () , quote ! { # pubkeys }) } else { let stream : proc_macro2 :: TokenStream = input . parse () ? ; return Err (syn :: Error :: new_spanned (stream , "unexpected token")) ; } ; Ok (Pubkeys { method , num , pubkeys , }) } }
};
}
