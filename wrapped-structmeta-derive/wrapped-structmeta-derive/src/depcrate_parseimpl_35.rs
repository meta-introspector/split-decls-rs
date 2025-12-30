// Generated macro for impl_35 (impl)
macro_rules! Depcrate_parseimpl_35 {
() => {
// Module: crate::parse
// Provides: {"impl_35"}
// Dependencies: {}
impl Parse for ParseAttributeArg { fn parse (input : ParseStream) -> Result < Self > { if input . peek (kw :: any) { Ok (Self :: Any (input . parse () ?)) } else if input . peek (kw :: peek) { Ok (Self :: Peek (input . parse () ?)) } else if input . peek (kw :: terminated) { Ok (Self :: Terminated (input . parse () ?)) } else if input . peek (kw :: dump) { Ok (Self :: Dump (input . parse () ?)) } else { Err (input . error ("expected `any`, `peek`, `terminated` or `dump`.")) } } }
};
}
