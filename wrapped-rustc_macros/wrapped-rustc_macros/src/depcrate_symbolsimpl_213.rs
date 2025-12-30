// Generated macro for impl_213 (impl)
macro_rules! Depcrate_symbolsimpl_213 {
() => {
// Module: crate::symbols
// Provides: {"impl_213"}
// Dependencies: {}
impl Parse for Keyword { fn parse (input : ParseStream < '_ >) -> Result < Self > { let name = input . parse () ? ; input . parse :: < Token ! [:] > () ? ; let value = input . parse () ? ; Ok (Keyword { name , value }) } }
};
}
