// Generated macro for deprecated_id_to_tokens (function)
macro_rules! Depcratedeprecated_id_to_tokens {
() => {
// Module: crate
// Provides: {"deprecated_id_to_tokens"}
// Dependencies: {}
fn deprecated_id_to_tokens (id : & proc_macro2 :: TokenStream , pubkey_type : proc_macro2 :: TokenStream , tokens : & mut proc_macro2 :: TokenStream ,) { tokens . extend (quote ! { # [doc = " The static program ID."] pub static ID : # pubkey_type = # id ; # [doc = " Returns `true` if given pubkey is the program ID."] # [deprecated ()] pub fn check_id (id : &# pubkey_type) -> bool { id == & ID } # [doc = " Returns the program ID."] # [deprecated ()] pub fn id () -> # pubkey_type { ID } # [cfg (test)] # [test] # [allow (deprecated)] fn test_id () { assert ! (check_id (& id ())) ; } }) ; }
};
}
