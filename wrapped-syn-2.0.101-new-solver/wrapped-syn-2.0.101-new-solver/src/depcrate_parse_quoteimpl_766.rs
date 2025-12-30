// Generated macro for impl_766 (impl)
macro_rules! Depcrate_parse_quoteimpl_766 {
() => {
// Module: crate::parse_quote
// Provides: {"impl_766"}
// Dependencies: {}
# [cfg (any (feature = "full" , feature = "derive"))] impl ParseQuote for Attribute { fn parse (input : ParseStream) -> Result < Self > { if input . peek (Token ! [#]) && input . peek2 (Token ! [!]) { attr :: parsing :: single_parse_inner (input) } else { attr :: parsing :: single_parse_outer (input) } } }
};
}
