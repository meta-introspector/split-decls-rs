// Generated macro for parse_pubkey (function)
macro_rules! Depcrateparse_pubkey {
() => {
// Module: crate
// Provides: {"parse_pubkey"}
// Dependencies: {}
fn parse_pubkey (id_literal : & LitStr , pubkey_type : & proc_macro2 :: TokenStream ,) -> Result < proc_macro2 :: TokenStream > { let id_vec = bs58 :: decode (id_literal . value ()) . into_vec () . map_err (| _ | syn :: Error :: new_spanned (id_literal , "failed to decode base58 string")) ? ; let id_array = < [u8 ; 32] > :: try_from (< & [u8] > :: clone (& & id_vec [..])) . map_err (| _ | { syn :: Error :: new_spanned (id_literal , format ! ("pubkey array is not 32 bytes long: len={}" , id_vec . len ()) ,) }) ? ; let bytes = id_array . iter () . map (| b | LitByte :: new (* b , Span :: call_site ())) ; Ok (quote ! { # pubkey_type :: new_from_array ([# (# bytes ,) *]) }) }
};
}
