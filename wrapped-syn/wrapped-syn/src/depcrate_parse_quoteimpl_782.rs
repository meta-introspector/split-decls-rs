// Generated macro for impl_782 (impl)
macro_rules! Depcrate_parse_quoteimpl_782 {
() => {
// Module: crate::parse_quote
// Provides: {"impl_782"}
// Dependencies: {}
# [cfg (any (feature = "full" , feature = "derive"))] impl ParseQuote for Attribute { fn parse (input : ParseStream) -> Result < Self > { if input . peek (Token ! [#]) && input . peek2 (Token ! [!]) { attr :: parsing :: single_parse_inner (input) } else { attr :: parsing :: single_parse_outer (input) } } }
};
}
