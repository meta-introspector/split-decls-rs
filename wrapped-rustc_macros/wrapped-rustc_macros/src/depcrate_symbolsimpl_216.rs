// Generated macro for impl_216 (impl)
macro_rules! Depcrate_symbolsimpl_216 {
() => {
// Module: crate::symbols
// Provides: {"impl_216"}
// Dependencies: {}
impl Parse for Symbol { fn parse (input : ParseStream < '_ >) -> Result < Self > { let name = input . parse () ? ; let colon_token : Option < Token ! [:] > = input . parse () ? ; let value = if colon_token . is_some () { input . parse () ? } else { Value :: SameAsName } ; Ok (Symbol { name , value }) } }
};
}
