// Generated macro for impl_762 (impl)
macro_rules! Depcrate_parse_quoteimpl_762 {
() => {
// Module: crate::parse_quote
// Provides: {"impl_762"}
// Dependencies: {}
impl < T : Parse > ParseQuote for T { fn parse (input : ParseStream) -> Result < Self > { < T as Parse > :: parse (input) } }
};
}
