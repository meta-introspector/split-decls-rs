// Generated macro for impl_10 (impl)
macro_rules! Depcrateimpl_10 {
() => {
// Module: crate
// Provides: {"impl_10"}
// Dependencies: {}
impl Parse for IdDeprecated { fn parse (input : ParseStream) -> Result < Self > { parse_id (input , quote ! { :: solana_sdk :: pubkey :: Pubkey }) . map (Self) } }
};
}
