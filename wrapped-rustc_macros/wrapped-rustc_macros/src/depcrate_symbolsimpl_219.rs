// Generated macro for impl_219 (impl)
macro_rules! Depcrate_symbolsimpl_219 {
() => {
// Module: crate::symbols
// Provides: {"impl_219"}
// Dependencies: {}
impl Parse for Input { fn parse (input : ParseStream < '_ >) -> Result < Self > { input . parse :: < kw :: Keywords > () ? ; let content ; braced ! (content in input) ; let keywords = Punctuated :: parse_terminated (& content) ? ; input . parse :: < kw :: Symbols > () ? ; let content ; braced ! (content in input) ; let symbols = Punctuated :: parse_terminated (& content) ? ; Ok (Input { keywords , symbols }) } }
};
}
