// Generated macro for impl_194 (impl)
macro_rules! Depcrate_to_tokensimpl_194 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_194"}
// Dependencies: {}
impl ToTokenTree for bool { fn into_token_tree (self) -> TokenTree { let lit = if self { "true" } else { "false" } ; TokenTree :: Ident (Ident :: new (lit , Span :: mixed_site ())) } }
};
}
