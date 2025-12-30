// Generated macro for impl_771 (impl)
macro_rules! Depcrate_parse_quoteimpl_771 {
() => {
// Module: crate::parse_quote
// Provides: {"impl_771"}
// Dependencies: {}
impl < T : Parse , P : Parse > ParseQuote for Punctuated < T , P > { fn parse (input : ParseStream) -> Result < Self > { Self :: parse_terminated (input) } }
};
}
