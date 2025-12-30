// Generated macro for id_to_tokens (function)
macro_rules! Depcrateid_to_tokens {
() => {
// Module: crate
// Provides: {"id_to_tokens"}
// Dependencies: {}
fn id_to_tokens (id : & proc_macro2 :: TokenStream , pubkey_type : proc_macro2 :: TokenStream , tokens : & mut proc_macro2 :: TokenStream ,) { tokens . extend (quote ! { # [doc = " The const program ID."] pub const ID : # pubkey_type = # id ; # [doc = " Returns `true` if given pubkey is the program ID."] pub fn check_id (id : &# pubkey_type) -> bool { id == & ID } # [doc = " Returns the program ID."] pub const fn id () -> # pubkey_type { ID } # [cfg (test)] # [test] fn test_id () { assert ! (check_id (& id ())) ; } }) ; }
};
}
