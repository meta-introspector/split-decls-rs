// Generated macro for impl_22 (impl)
macro_rules! Depcrate_attrimpl_22 {
() => {
// Module: crate::attr
// Provides: {"impl_22"}
// Dependencies: {}
impl ToTokens for LitStrOrIdent { fn to_tokens (& self , tokens : & mut TokenStream) { match self { LitStrOrIdent :: LitStr (target) => target . to_tokens (tokens) , LitStrOrIdent :: Ident (ident) => ident . to_tokens (tokens) , } } }
};
}
