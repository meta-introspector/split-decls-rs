// Generated macro for impl_111 (impl)
macro_rules! Depcrate_to_tokens_attributeimpl_111 {
() => {
// Module: crate::to_tokens_attribute
// Provides: {"impl_111"}
// Dependencies: {}
impl Parse for ToTokensAttribute { fn parse (input : syn :: parse :: ParseStream) -> Result < Self > { let args = input . parse_terminated (ToTokensAttributeArg :: parse , Token ! [,]) ? ; let mut token = Vec :: new () ; let mut dump = None ; for arg in args . into_iter () { match arg { ToTokensAttributeArg :: Token (token_value) => { token . push (token_value) ; } ToTokensAttributeArg :: Dump (kw_dump) => { if dump . is_none () { dump = Some (kw_dump . span ()) ; } } } } Ok (Self { dump , token }) } }
};
}
