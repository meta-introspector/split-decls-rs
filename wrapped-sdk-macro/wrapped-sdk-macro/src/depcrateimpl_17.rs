// Generated macro for impl_17 (impl)
macro_rules! Depcrateimpl_17 {
() => {
// Module: crate
// Provides: {"impl_17"}
// Dependencies: {}
impl ToTokens for Pubkeys { fn to_tokens (& self , tokens : & mut proc_macro2 :: TokenStream) { let Pubkeys { method , num , pubkeys , } = self ; let pubkey_type = quote ! { :: solana_sdk :: pubkey :: Pubkey } ; if * num == 1 { tokens . extend (quote ! { pub fn # method () -> # pubkey_type { # pubkeys } }) ; } else { tokens . extend (quote ! { pub fn # method () -> :: std :: vec :: Vec <# pubkey_type > { vec ! [# pubkeys] } }) ; } } }
};
}
