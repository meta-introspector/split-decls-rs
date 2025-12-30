// Generated macro for impl_11 (impl)
macro_rules! Depcrateimpl_11 {
() => {
// Module: crate
// Provides: {"impl_11"}
// Dependencies: {}
impl ToTokens for IdDeprecated { fn to_tokens (& self , tokens : & mut proc_macro2 :: TokenStream) { deprecated_id_to_tokens (& self . 0 , quote ! { :: solana_sdk :: pubkey :: Pubkey } , tokens) } }
};
}
