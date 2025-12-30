// Generated macro for parse_id (function)
macro_rules! Depcrateparse_id {
() => {
// Module: crate
// Provides: {"parse_id"}
// Dependencies: {}
fn parse_id (input : ParseStream , pubkey_type : proc_macro2 :: TokenStream ,) -> Result < proc_macro2 :: TokenStream > { let id = if input . peek (syn :: LitStr) { let id_literal : LitStr = input . parse () ? ; parse_pubkey (& id_literal , & pubkey_type) ? } else { let expr : Expr = input . parse () ? ; quote ! { # expr } } ; if ! input . is_empty () { let stream : proc_macro2 :: TokenStream = input . parse () ? ; return Err (syn :: Error :: new_spanned (stream , "unexpected token")) ; } Ok (id) }
};
}
