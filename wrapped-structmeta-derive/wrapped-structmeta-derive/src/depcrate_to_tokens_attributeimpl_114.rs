// Generated macro for impl_114 (impl)
macro_rules! Depcrate_to_tokens_attributeimpl_114 {
() => {
// Module: crate::to_tokens_attribute
// Provides: {"impl_114"}
// Dependencies: {}
impl Parse for ToTokensAttributeArg { fn parse (input : syn :: parse :: ParseStream) -> Result < Self > { if input . peek (LitStr) { Ok (Self :: Token (input . parse () ?)) } else if input . peek (kw :: dump) { Ok (Self :: Dump (input . parse () ?)) } else { Err (input . error ("expected string literal.")) } } }
};
}
