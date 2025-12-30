// Generated macro for impl_767 (impl)
macro_rules! Depcrate_parse_quoteimpl_767 {
() => {
// Module: crate::parse_quote
// Provides: {"impl_767"}
// Dependencies: {}
# [cfg (any (feature = "full" , feature = "derive"))] impl ParseQuote for Vec < Attribute > { fn parse (input : ParseStream) -> Result < Self > { let mut attrs = Vec :: new () ; while ! input . is_empty () { attrs . push (ParseQuote :: parse (input) ?) ; } Ok (attrs) } }
};
}
