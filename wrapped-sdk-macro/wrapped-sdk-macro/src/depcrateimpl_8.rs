// Generated macro for impl_8 (impl)
macro_rules! Depcrateimpl_8 {
() => {
// Module: crate
// Provides: {"impl_8"}
// Dependencies: {}
impl ToTokens for Id { fn to_tokens (& self , tokens : & mut proc_macro2 :: TokenStream) { id_to_tokens (& self . 0 , quote ! { :: solana_sdk :: pubkey :: Pubkey } , tokens) } }
};
}
