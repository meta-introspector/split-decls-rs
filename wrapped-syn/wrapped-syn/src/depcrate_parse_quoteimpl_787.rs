// Generated macro for impl_787 (impl)
macro_rules! Depcrate_parse_quoteimpl_787 {
() => {
// Module: crate::parse_quote
// Provides: {"impl_787"}
// Dependencies: {}
impl < T : Parse , P : Parse > ParseQuote for Punctuated < T , P > { fn parse (input : ParseStream) -> Result < Self > { Self :: parse_terminated (input) } }
};
}
