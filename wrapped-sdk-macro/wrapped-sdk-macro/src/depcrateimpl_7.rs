// Generated macro for impl_7 (impl)
macro_rules! Depcrateimpl_7 {
() => {
// Module: crate
// Provides: {"impl_7"}
// Dependencies: {}
impl Parse for Id { fn parse (input : ParseStream) -> Result < Self > { parse_id (input , quote ! { :: solana_sdk :: pubkey :: Pubkey }) . map (Self) } }
};
}
