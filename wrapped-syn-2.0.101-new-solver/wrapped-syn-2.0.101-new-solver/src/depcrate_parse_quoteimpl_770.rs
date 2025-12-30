// Generated macro for impl_770 (impl)
macro_rules! Depcrate_parse_quoteimpl_770 {
() => {
// Module: crate::parse_quote
// Provides: {"impl_770"}
// Dependencies: {}
# [cfg (feature = "full")] impl ParseQuote for Box < Pat > { fn parse (input : ParseStream) -> Result < Self > { < Pat as ParseQuote > :: parse (input) . map (Box :: new) } }
};
}
